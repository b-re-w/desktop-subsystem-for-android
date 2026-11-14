package io.github.thisisthepy.dsa.agent.platform

import android.content.ComponentName
import android.content.Context
import android.content.pm.ApplicationInfo
import android.content.pm.PackageManager
import android.content.res.Configuration
import android.content.res.Resources
import android.graphics.drawable.Drawable
import android.hardware.display.DisplayManager
import android.os.HandlerThread
import android.os.IBinder
import android.os.ServiceManager
import android.view.Display
import android.view.Gravity
import android.view.View
import android.view.WindowManager
import android.widget.TextView
import io.github.thisisthepy.dsa.agent.Log
import io.github.thisisthepy.dsa.agent.tasks.TaskBounds
import io.github.thisisthepy.dsa.agent.tasks.TaskSnapshot
import io.github.thisisthepy.dsa.agent.toasts.ShownToast
import io.github.thisisthepy.dsa.agent.toasts.TextToasts
import io.github.thisisthepy.dsa.agent.toasts.ToastCalls
import io.github.thisisthepy.dsa.agent.toasts.ToastPlacement
import io.github.thisisthepy.dsa.agent.toasts.ToastRequest
import io.github.thisisthepy.dsa.agent.toasts.ToastWindows
import io.github.thisisthepy.dsa.agent.toasts.fillsHeight
import io.github.thisisthepy.dsa.agent.toasts.fillsWidth
import io.github.thisisthepy.dsa.agent.toasts.toastFrame
import io.github.thisisthepy.dsa.agent.toasts.toastKeepsTwoLines
import io.github.thisisthepy.dsa.agent.toasts.toastShowsIcon

/**
 * Draws the text toasts the notification manager sends to the status bar, each inside the
 * window of its app's task ([TextToasts]).
 *
 * On a phone SystemUI draws them. It makes its own view and hands it to the framework's
 * ToastPresenter, which adds the toast window on the window token the notification manager
 * made, sends the accessibility event, tells the app's callback, and on hiding gives the token
 * back (INotificationManager.finishToken). The agent uses the same presenter, so all of that is
 * the framework's own code, and draws the view the framework draws for a toast an app shows
 * itself (ToastPresenter.getTextToastView), with the app's resources and theme, so both kinds
 * of toast look alike and the image's overlay styles both.
 *
 * - **Whose window it is.** The window manager shows a toast window only when its package
 *   belongs to the process that adds it (the toast app-op is checked for the pair), so the
 *   presenter's context is the agent's and only the view's context is the app's.
 * - **Where.** Every task is a freeform window on one display, and a toast window is laid out
 *   on the display, not in a task. The toast is measured here, placed where the framework's
 *   default gravity and offset put it in the task's bounds ([toastFrame]), and added at that
 *   exact place and size. It fits no insets, so its place is counted from the display's
 *   corner.
 * - **At the task's density.** The task draws at its host monitor's density, which is not the
 *   display's, and its frames are shown one pixel to one screen pixel, so the toast is made at
 *   the task's density.
 * - **Touches pass.** The window is marked a trusted overlay, as SystemUI's is (the agent holds
 *   INTERNAL_SYSTEM_WINDOW): Android drops a touch that passes through another app's window
 *   to the app under it, and a toast is not touchable.
 *
 * ToastPresenter and the binder interfaces it takes are not in the SDK; they are reached by
 * reflection, which a platform app may do. Everything runs on a thread of its own, made with
 * the first toast, so a slow app theme never holds up the agent's work.
 */
class AndroidToasts(
    private val context: Context,
    private val log: Log,
    tasks: () -> List<TaskSnapshot>,
) : ToastCalls {
    private val scheduler by lazy { HandlerScheduler(HandlerThread("dsa-toasts").apply { start() }.looper) }
    private val toasts by lazy { TextToasts(Windows(), tasks, scheduler, log, context.packageName) }
    private var found: Framework? = null
    private var warnedTrust = false

    override fun show(request: ToastRequest) = scheduler.post { toasts.show(request) }

    override fun hide(packageName: String, token: Any?) = scheduler.post { toasts.hide(packageName, token) }

    private inner class Windows : ToastWindows {
        override fun show(request: ToastRequest, task: TaskSnapshot): ShownToast? = try {
            draw(request, task)
        } catch (e: ReflectiveOperationException) {
            log.warn("this framework could not draw the toast of ${request.packageName}", e.cause ?: e)
            null
        } catch (e: PackageManager.NameNotFoundException) {
            log.info("the app of a toast is gone: ${request.packageName}")
            null
        }
    }

    private fun draw(request: ToastRequest, task: TaskSnapshot): ShownToast? {
        val framework = found ?: Framework().also { found = it }
        val display = context.getSystemService(DisplayManager::class.java).getDisplay(request.displayId)
        if (display == null) {
            log.info("the display of the toast of ${request.packageName} is gone")
            return null
        }
        val app = context.packageManager.getApplicationInfo(request.packageName, 0)
        val themed = themedContext(app, task, display)
        val view = toastView(framework, themed, app, request.text)
        val own = context.createDisplayContext(display)
        val presenter = framework.constructor.newInstance(own, framework.accessibility, framework.notifications, request.packageName)
        val params = framework.layoutParams.invoke(presenter) as WindowManager.LayoutParams
        params.setFitInsetsTypes(0)
        trust(params)
        val shown = Shown(framework, presenter, own, themed, view, params, request.callback as IBinder?)
        val frame = shown.measure(task)
        framework.show.invoke(
            presenter, view, request.token as IBinder?, request.windowToken as IBinder?, request.duration,
            Gravity.TOP or Gravity.LEFT, frame.left, frame.top, 0f, 0f, framework.callback(shown.callback),
        )
        return shown
    }

    /**
     * The app's resources on the toast's display, at the task's density, with the theme the
     * app would draw a toast with: its task's top activity's, else the app's, else the device
     * default, which follows the dark theme. A restricted context loads no font or code of the
     * app's into the agent.
     */
    private fun themedContext(app: ApplicationInfo, task: TaskSnapshot, display: Display): Context {
        var themed = context.createPackageContext(app.packageName, Context.CONTEXT_RESTRICTED).createDisplayContext(display)
        if (task.densityDpi > 0 && task.densityDpi != themed.resources.configuration.densityDpi) {
            val density = Configuration()
            density.densityDpi = task.densityDpi
            themed = themed.createConfigurationContext(density)
        }
        themed.setTheme(theme(app, task))
        return themed
    }

    private fun theme(app: ApplicationInfo, task: TaskSnapshot): Int {
        val top = task.stack?.top?.let { ComponentName.unflattenFromString(it) }
        // A theme is a resource of its own package: another app's activity names none of this one's.
        if (top != null && top.packageName == app.packageName) {
            try {
                val theme = context.packageManager.getActivityInfo(top, 0).themeResource
                if (theme != 0) return theme
            } catch (e: PackageManager.NameNotFoundException) {
                // The app's theme, below.
            }
        }
        return if (app.theme != 0) app.theme else android.R.style.Theme_DeviceDefault_DayNight
    }

    /**
     * The framework's own text toast view, with the app's icon where Android shows one and
     * without the limit of two lines where Android has none.
     */
    private fun toastView(framework: Framework, themed: Context, app: ApplicationInfo, text: CharSequence): View {
        val packages = context.packageManager
        val systemApp = app.flags and (ApplicationInfo.FLAG_SYSTEM or ApplicationInfo.FLAG_UPDATED_SYSTEM_APP) != 0
        val withIcon = framework.iconView
        val icon: Drawable? = if (withIcon != null &&
            toastShowsIcon(app.targetSdkVersion, systemApp, packages.getLaunchIntentForPackage(app.packageName) != null)
        ) {
            try {
                packages.getApplicationIcon(app)
            } catch (e: RuntimeException) {
                null
            }
        } else {
            null
        }
        val view = if (withIcon != null && icon != null) {
            withIcon.invoke(null, themed, text, icon) as View
        } else {
            framework.textView.invoke(null, themed, text) as View
        }
        if (!toastKeepsTwoLines(app.targetSdkVersion)) {
            view.findViewById<TextView>(android.R.id.message)?.maxLines = Int.MAX_VALUE
        }
        return view
    }

    /** Lets touches through to the app under the toast (WindowManager.LayoutParams.setTrustedOverlay). */
    private fun trust(params: WindowManager.LayoutParams) {
        try {
            WindowManager.LayoutParams::class.java.getMethod("setTrustedOverlay").invoke(params)
        } catch (e: ReflectiveOperationException) {
            if (!warnedTrust) {
                warnedTrust = true
                log.warn("could not mark a toast a trusted overlay; a touch on it may not reach the app", e.cause ?: e)
            }
        }
    }

    /** One toast window. The presenter keeps its context weakly, so it is held here. */
    private inner class Shown(
        private val framework: Framework,
        private val presenter: Any,
        private val own: Context,
        private val themed: Context,
        private val view: View,
        private val params: WindowManager.LayoutParams,
        val callback: IBinder?,
    ) : ShownToast {
        private val placement = defaultPlacement()

        /**
         * Measures the toast for [task], gives the window that size, and returns its frame on
         * the display. The framework's layout keeps a toast 16 dp from the screen's sides
         * when the screen is narrower than the toast's largest width; here the screen is the
         * window.
         */
        fun measure(task: TaskSnapshot): TaskBounds {
            val bounds = task.bounds
            val margin = (SIDE_MARGIN_DP * themed.resources.displayMetrics.density + 0.5f).toInt()
            val width = maxOf(bounds.right - bounds.left - 2 * margin, 1)
            val height = maxOf(bounds.bottom - bounds.top, 1)
            view.measure(
                View.MeasureSpec.makeMeasureSpec(width, if (fillsWidth(placement.gravity)) View.MeasureSpec.EXACTLY else View.MeasureSpec.AT_MOST),
                View.MeasureSpec.makeMeasureSpec(height, if (fillsHeight(placement.gravity)) View.MeasureSpec.EXACTLY else View.MeasureSpec.AT_MOST),
            )
            val frame = toastFrame(bounds, view.measuredWidth, view.measuredHeight, placement)
            params.width = frame.right - frame.left
            params.height = frame.bottom - frame.top
            return frame
        }

        override fun place(task: TaskSnapshot) {
            val frame = measure(task)
            params.x = frame.left
            params.y = frame.top
            if (view.parent != null) own.getSystemService(WindowManager::class.java).updateViewLayout(view, params)
        }

        override fun hide() {
            try {
                framework.hide.invoke(presenter, framework.callback(callback))
            } catch (e: ReflectiveOperationException) {
                throw IllegalStateException("the framework could not hide a toast", e.cause ?: e)
            }
        }

        /**
         * The framework's default place of a toast, read from the app's resources as Toast
         * reads it. A text toast carries no place of the app's own: Android ignores
         * setGravity and setMargin on one and sends the status bar neither.
         */
        private fun defaultPlacement(): ToastPlacement {
            val resources = themed.resources
            val system = Resources.getSystem()
            val gravityId = system.getIdentifier("config_toastDefaultGravity", "integer", "android")
            val offsetId = system.getIdentifier("toast_y_offset", "dimen", "android")
            val gravity = if (gravityId != 0) resources.getInteger(gravityId) else Gravity.CENTER_HORIZONTAL or Gravity.BOTTOM
            val offset = if (offsetId != 0) {
                resources.getDimensionPixelSize(offsetId)
            } else {
                (DEFAULT_Y_OFFSET_DP * resources.displayMetrics.density + 0.5f).toInt()
            }
            return ToastPlacement(Gravity.getAbsoluteGravity(gravity, resources.configuration.layoutDirection), 0, offset)
        }
    }

    /** The framework's toast classes, found once. */
    private class Framework {
        private val presenter = Class.forName("android.widget.ToastPresenter")
        private val callbackType = Class.forName("android.app.ITransientNotificationCallback")
        private val accessibilityType = Class.forName("android.view.accessibility.IAccessibilityManager")
        private val notificationsType = Class.forName("android.app.INotificationManager")

        val constructor = presenter.getConstructor(Context::class.java, accessibilityType, notificationsType, String::class.java)
        val show = presenter.getMethod(
            "show", View::class.java, IBinder::class.java, IBinder::class.java,
            Int::class.javaPrimitiveType, Int::class.javaPrimitiveType, Int::class.javaPrimitiveType, Int::class.javaPrimitiveType,
            Float::class.javaPrimitiveType, Float::class.javaPrimitiveType, callbackType,
        )
        val hide = presenter.getMethod("hide", callbackType)
        val layoutParams = presenter.getMethod("getLayoutParams")
        val textView = presenter.getMethod("getTextToastView", Context::class.java, CharSequence::class.java)
        val iconView = try {
            presenter.getMethod("getTextToastViewWithIcon", Context::class.java, CharSequence::class.java, Drawable::class.java)
        } catch (e: NoSuchMethodException) {
            null
        }
        val accessibility = asInterface(accessibilityType, ServiceManager.getService(Context.ACCESSIBILITY_SERVICE))
        val notifications = asInterface(notificationsType, ServiceManager.getService(Context.NOTIFICATION_SERVICE))

        /** The app's ITransientNotificationCallback; null when it listens to nothing. */
        fun callback(binder: IBinder?): Any? = if (binder == null) null else asInterface(callbackType, binder)

        private fun asInterface(type: Class<*>, binder: IBinder?): Any? =
            Class.forName("${type.name}\$Stub").getMethod("asInterface", IBinder::class.java).invoke(null, binder)
    }

    private companion object {
        // The side margins of the framework's toast layout (transient_notification).
        const val SIDE_MARGIN_DP = 16

        // The framework's toast_y_offset, for a framework that names it otherwise.
        const val DEFAULT_Y_OFFSET_DP = 24
    }
}
