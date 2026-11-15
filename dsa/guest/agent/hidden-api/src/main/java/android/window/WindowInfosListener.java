package android.window;

import android.util.Pair;
import android.view.InputWindowHandle;

/**
 * The framework's listener for SurfaceFlinger's list of windows: every window that is on a
 * display, front to back, each time any of them changes. Registering needs
 * ACCESS_SURFACE_FLINGER. Declarations only.
 */
public abstract class WindowInfosListener {
    public WindowInfosListener() {
        throw new RuntimeException("Stub!");
    }

    /** Called on a binder thread; SurfaceFlinger waits for it to return before its next list. */
    public abstract void onWindowInfosChanged(InputWindowHandle[] windowHandles, DisplayInfo[] displayInfos);

    /** Starts listening, and returns the windows and displays as they are now. */
    public Pair<InputWindowHandle[], DisplayInfo[]> register() {
        throw new RuntimeException("Stub!");
    }

    public void unregister() {
        throw new RuntimeException("Stub!");
    }

    /** One display of the list. Only named here; the agent reads nothing of it. */
    public static final class DisplayInfo {
        DisplayInfo() {
            throw new RuntimeException("Stub!");
        }
    }
}
