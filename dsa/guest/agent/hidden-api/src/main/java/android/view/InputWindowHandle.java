package android.view;

import android.graphics.Rect;
import android.os.IBinder;

/**
 * One window as SurfaceFlinger lists it for a WindowInfosListener: the fields the agent reads
 * to find the app overlay windows. Declarations only; the class has more fields than these.
 */
public final class InputWindowHandle {
    /** The input channel's token; null for a window without one. */
    public IBinder token;
    /** The window's identity hash and its title, or its package when it has no title. */
    public String name;
    /** WindowManager.LayoutParams.flags and .type. */
    public int layoutParamsFlags;
    public int layoutParamsType;
    /** The window's frame on the display, without its surface insets. */
    public final Rect frame = new Rect();
    /** How far the window's surface reaches past its frame on every side. */
    public int surfaceInset;
    /** android.os.InputConfig bits: NOT_VISIBLE, NOT_FOCUSABLE, NOT_TOUCHABLE and others. */
    public int inputConfig;
    public int ownerPid;
    public int ownerUid;
    public String packageName;
    public int displayId;
    public float alpha;

    InputWindowHandle() {
        throw new RuntimeException("Stub!");
    }

    /** The window's client token, which names it for as long as it lives. */
    public IBinder getWindowToken() {
        throw new RuntimeException("Stub!");
    }
}
