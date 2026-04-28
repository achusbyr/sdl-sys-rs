//! Generated constants

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropertyType { Pointer, String, Number, Float, Boolean }

#[derive(Debug, Clone, Copy)]
pub struct Hint {
    pub name: &'static str,
    pub value: &'static str,
    pub doc: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct Property {
    pub name: &'static str,
    pub value: &'static str,
    pub ty: PropertyType,
    pub doc: &'static str,
}

/**
* Specify the behavior of Alt+Tab while the keyboard is grabbed.
*
* By default, SDL emulates Alt+Tab functionality while the keyboard is
* grabbed and your window is full-screen. This prevents the user from getting
* stuck in your application if you've enabled keyboard grab.
*
* The variable can be set to the following values:
*
* - "0": SDL will not handle Alt+Tab. Your application is responsible for
*   handling Alt+Tab while the keyboard is grabbed.
* - "1": SDL will minimize your window when Alt+Tab is pressed (default)
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_ALLOW_ALT_TAB_WHILE_GRABBED: Hint = Hint { name: "SDL_HINT_ALLOW_ALT_TAB_WHILE_GRABBED", value: "SDL_ALLOW_ALT_TAB_WHILE_GRABBED", doc: "/**\n* Specify the behavior of Alt+Tab while the keyboard is grabbed.\n*\n* By default, SDL emulates Alt+Tab functionality while the keyboard is\n* grabbed and your window is full-screen. This prevents the user from getting\n* stuck in your application if you've enabled keyboard grab.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": SDL will not handle Alt+Tab. Your application is responsible for\n*   handling Alt+Tab while the keyboard is grabbed.\n* - \"1\": SDL will minimize your window when Alt+Tab is pressed (default)\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable to control whether the SDL activity is allowed to be re-created.
*
* If this hint is true, the activity can be recreated on demand by the OS,
* and Java static data and C++ static data remain with their current values.
* If this hint is false, then SDL will call exit() when you return from your
* main function and the application will be terminated and then started fresh
* each time.
*
* The variable can be set to the following values:
*
* - "0": The application starts fresh at each launch. (default)
* - "1": The application activity can be recreated by the OS.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_ANDROID_ALLOW_RECREATE_ACTIVITY: Hint = Hint { name: "SDL_HINT_ANDROID_ALLOW_RECREATE_ACTIVITY", value: "SDL_ANDROID_ALLOW_RECREATE_ACTIVITY", doc: "/**\n* A variable to control whether the SDL activity is allowed to be re-created.\n*\n* If this hint is true, the activity can be recreated on demand by the OS,\n* and Java static data and C++ static data remain with their current values.\n* If this hint is false, then SDL will call exit() when you return from your\n* main function and the application will be terminated and then started fresh\n* each time.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": The application starts fresh at each launch. (default)\n* - \"1\": The application activity can be recreated by the OS.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable to control whether the event loop will block itself when the app
* is paused.
*
* The variable can be set to the following values:
*
* - "0": Non blocking.
* - "1": Blocking. (default)
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_ANDROID_BLOCK_ON_PAUSE: Hint = Hint { name: "SDL_HINT_ANDROID_BLOCK_ON_PAUSE", value: "SDL_ANDROID_BLOCK_ON_PAUSE", doc: "/**\n* A variable to control whether the event loop will block itself when the app\n* is paused.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Non blocking.\n* - \"1\": Blocking. (default)\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable to control whether low latency audio should be enabled.
*
* Some devices have poor quality output when this is enabled, but this is
* usually an improvement in audio latency.
*
* The variable can be set to the following values:
*
* - "0": Low latency audio is not enabled.
* - "1": Low latency audio is enabled. (default)
*
* This hint should be set before SDL audio is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_ANDROID_LOW_LATENCY_AUDIO: Hint = Hint { name: "SDL_HINT_ANDROID_LOW_LATENCY_AUDIO", value: "SDL_ANDROID_LOW_LATENCY_AUDIO", doc: "/**\n* A variable to control whether low latency audio should be enabled.\n*\n* Some devices have poor quality output when this is enabled, but this is\n* usually an improvement in audio latency.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Low latency audio is not enabled.\n* - \"1\": Low latency audio is enabled. (default)\n*\n* This hint should be set before SDL audio is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable to control whether we trap the Android back button to handle it
* manually.
*
* This is necessary for the right mouse button to work on some Android
* devices, or to be able to trap the back button for use in your code
* reliably. If this hint is true, the back button will show up as an
* SDL_EVENT_KEY_DOWN / SDL_EVENT_KEY_UP pair with a keycode of
* SDL_SCANCODE_AC_BACK.
*
* The variable can be set to the following values:
*
* - "0": Back button will be handled as usual for system. (default)
* - "1": Back button will be trapped, allowing you to handle the key press
*   manually. (This will also let right mouse click work on systems where the
*   right mouse button functions as back.)
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_ANDROID_TRAP_BACK_BUTTON: Hint = Hint { name: "SDL_HINT_ANDROID_TRAP_BACK_BUTTON", value: "SDL_ANDROID_TRAP_BACK_BUTTON", doc: "/**\n* A variable to control whether we trap the Android back button to handle it\n* manually.\n*\n* This is necessary for the right mouse button to work on some Android\n* devices, or to be able to trap the back button for use in your code\n* reliably. If this hint is true, the back button will show up as an\n* SDL_EVENT_KEY_DOWN / SDL_EVENT_KEY_UP pair with a keycode of\n* SDL_SCANCODE_AC_BACK.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Back button will be handled as usual for system. (default)\n* - \"1\": Back button will be trapped, allowing you to handle the key press\n*   manually. (This will also let right mouse click work on systems where the\n*   right mouse button functions as back.)\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable setting the app ID string.
*
* This string is used by desktop compositors to identify and group windows
* together, as well as match applications with associated desktop settings
* and icons.
*
* This will override SDL_PROP_APP_METADATA_IDENTIFIER_STRING, if set by the
* application.
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_APP_ID: Hint = Hint { name: "SDL_HINT_APP_ID", value: "SDL_APP_ID", doc: "/**\n* A variable setting the app ID string.\n*\n* This string is used by desktop compositors to identify and group windows\n* together, as well as match applications with associated desktop settings\n* and icons.\n*\n* This will override SDL_PROP_APP_METADATA_IDENTIFIER_STRING, if set by the\n* application.\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable setting the application name.
*
* This hint lets you specify the application name sent to the OS when
* required. For example, this will often appear in volume control applets for
* audio streams, and in lists of applications which are inhibiting the
* screensaver. You should use a string that describes your program ("My Game
* 2: The Revenge")
*
* This will override SDL_PROP_APP_METADATA_NAME_STRING, if set by the
* application.
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_APP_NAME: Hint = Hint { name: "SDL_HINT_APP_NAME", value: "SDL_APP_NAME", doc: "/**\n* A variable setting the application name.\n*\n* This hint lets you specify the application name sent to the OS when\n* required. For example, this will often appear in volume control applets for\n* audio streams, and in lists of applications which are inhibiting the\n* screensaver. You should use a string that describes your program (\"My Game\n* 2: The Revenge\")\n*\n* This will override SDL_PROP_APP_METADATA_NAME_STRING, if set by the\n* application.\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether controllers used with the Apple TV generate
* UI events.
*
* When UI events are generated by controller input, the app will be
* backgrounded when the Apple TV remote's menu button is pressed, and when
* the pause or B buttons on gamepads are pressed.
*
* More information about properly making use of controllers for the Apple TV
* can be found here:
* https://developer.apple.com/tvos/human-interface-guidelines/remote-and-controllers/
*
* The variable can be set to the following values:
*
* - "0": Controller input does not generate UI events. (default)
* - "1": Controller input generates UI events.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_APPLE_TV_CONTROLLER_UI_EVENTS: Hint = Hint { name: "SDL_HINT_APPLE_TV_CONTROLLER_UI_EVENTS", value: "SDL_APPLE_TV_CONTROLLER_UI_EVENTS", doc: "/**\n* A variable controlling whether controllers used with the Apple TV generate\n* UI events.\n*\n* When UI events are generated by controller input, the app will be\n* backgrounded when the Apple TV remote's menu button is pressed, and when\n* the pause or B buttons on gamepads are pressed.\n*\n* More information about properly making use of controllers for the Apple TV\n* can be found here:\n* https://developer.apple.com/tvos/human-interface-guidelines/remote-and-controllers/\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Controller input does not generate UI events. (default)\n* - \"1\": Controller input generates UI events.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the Apple TV remote's joystick axes will
* automatically match the rotation of the remote.
*
* The variable can be set to the following values:
*
* - "0": Remote orientation does not affect joystick axes. (default)
* - "1": Joystick axes are based on the orientation of the remote.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_APPLE_TV_REMOTE_ALLOW_ROTATION: Hint = Hint { name: "SDL_HINT_APPLE_TV_REMOTE_ALLOW_ROTATION", value: "SDL_APPLE_TV_REMOTE_ALLOW_ROTATION", doc: "/**\n* A variable controlling whether the Apple TV remote's joystick axes will\n* automatically match the rotation of the remote.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Remote orientation does not affect joystick axes. (default)\n* - \"1\": Joystick axes are based on the orientation of the remote.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* Specify the default ALSA audio device name.
*
* This variable is a specific audio device to open when the "default" audio
* device is used.
*
* This hint will be ignored when opening the default playback device if
* SDL_HINT_AUDIO_ALSA_DEFAULT_PLAYBACK_DEVICE is set, or when opening the
* default recording device if SDL_HINT_AUDIO_ALSA_DEFAULT_RECORDING_DEVICE is
* set.
*
* This hint should be set before an audio device is opened.
*
* \since This hint is available since SDL 3.2.0.
*
* \sa SDL_HINT_AUDIO_ALSA_DEFAULT_PLAYBACK_DEVICE
* \sa SDL_HINT_AUDIO_ALSA_DEFAULT_RECORDING_DEVICE
*/
pub const SDL_HINT_AUDIO_ALSA_DEFAULT_DEVICE: Hint = Hint { name: "SDL_HINT_AUDIO_ALSA_DEFAULT_DEVICE", value: "SDL_AUDIO_ALSA_DEFAULT_DEVICE", doc: "/**\n* Specify the default ALSA audio device name.\n*\n* This variable is a specific audio device to open when the \"default\" audio\n* device is used.\n*\n* This hint will be ignored when opening the default playback device if\n* SDL_HINT_AUDIO_ALSA_DEFAULT_PLAYBACK_DEVICE is set, or when opening the\n* default recording device if SDL_HINT_AUDIO_ALSA_DEFAULT_RECORDING_DEVICE is\n* set.\n*\n* This hint should be set before an audio device is opened.\n*\n* \\since This hint is available since SDL 3.2.0.\n*\n* \\sa SDL_HINT_AUDIO_ALSA_DEFAULT_PLAYBACK_DEVICE\n* \\sa SDL_HINT_AUDIO_ALSA_DEFAULT_RECORDING_DEVICE\n*/\n" };
/**
* Specify the default ALSA audio playback device name.
*
* This variable is a specific audio device to open for playback, when the
* "default" audio device is used.
*
* If this hint isn't set, SDL will check SDL_HINT_AUDIO_ALSA_DEFAULT_DEVICE
* before choosing a reasonable default.
*
* This hint should be set before an audio device is opened.
*
* \since This hint is available since SDL 3.2.0.
*
* \sa SDL_HINT_AUDIO_ALSA_DEFAULT_RECORDING_DEVICE
* \sa SDL_HINT_AUDIO_ALSA_DEFAULT_DEVICE
*/
pub const SDL_HINT_AUDIO_ALSA_DEFAULT_PLAYBACK_DEVICE: Hint = Hint { name: "SDL_HINT_AUDIO_ALSA_DEFAULT_PLAYBACK_DEVICE", value: "SDL_AUDIO_ALSA_DEFAULT_PLAYBACK_DEVICE", doc: "/**\n* Specify the default ALSA audio playback device name.\n*\n* This variable is a specific audio device to open for playback, when the\n* \"default\" audio device is used.\n*\n* If this hint isn't set, SDL will check SDL_HINT_AUDIO_ALSA_DEFAULT_DEVICE\n* before choosing a reasonable default.\n*\n* This hint should be set before an audio device is opened.\n*\n* \\since This hint is available since SDL 3.2.0.\n*\n* \\sa SDL_HINT_AUDIO_ALSA_DEFAULT_RECORDING_DEVICE\n* \\sa SDL_HINT_AUDIO_ALSA_DEFAULT_DEVICE\n*/\n" };
/**
* Specify the default ALSA audio recording device name.
*
* This variable is a specific audio device to open for recording, when the
* "default" audio device is used.
*
* If this hint isn't set, SDL will check SDL_HINT_AUDIO_ALSA_DEFAULT_DEVICE
* before choosing a reasonable default.
*
* This hint should be set before an audio device is opened.
*
* \since This hint is available since SDL 3.2.0.
*
* \sa SDL_HINT_AUDIO_ALSA_DEFAULT_PLAYBACK_DEVICE
* \sa SDL_HINT_AUDIO_ALSA_DEFAULT_DEVICE
*/
pub const SDL_HINT_AUDIO_ALSA_DEFAULT_RECORDING_DEVICE: Hint = Hint { name: "SDL_HINT_AUDIO_ALSA_DEFAULT_RECORDING_DEVICE", value: "SDL_AUDIO_ALSA_DEFAULT_RECORDING_DEVICE", doc: "/**\n* Specify the default ALSA audio recording device name.\n*\n* This variable is a specific audio device to open for recording, when the\n* \"default\" audio device is used.\n*\n* If this hint isn't set, SDL will check SDL_HINT_AUDIO_ALSA_DEFAULT_DEVICE\n* before choosing a reasonable default.\n*\n* This hint should be set before an audio device is opened.\n*\n* \\since This hint is available since SDL 3.2.0.\n*\n* \\sa SDL_HINT_AUDIO_ALSA_DEFAULT_PLAYBACK_DEVICE\n* \\sa SDL_HINT_AUDIO_ALSA_DEFAULT_DEVICE\n*/\n" };
/**
* A variable controlling the audio category on iOS and macOS.
*
* The variable can be set to the following values:
*
* - "ambient": Use the AVAudioSessionCategoryAmbient audio category, will be
*   muted by the phone mute switch (default)
* - "playback": Use the AVAudioSessionCategoryPlayback category.
*
* For more information, see Apple's documentation:
* https://developer.apple.com/library/content/documentation/Audio/Conceptual/AudioSessionProgrammingGuide/AudioSessionCategoriesandModes/AudioSessionCategoriesandModes.html
*
* This hint should be set before an audio device is opened.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_AUDIO_CATEGORY: Hint = Hint { name: "SDL_HINT_AUDIO_CATEGORY", value: "SDL_AUDIO_CATEGORY", doc: "/**\n* A variable controlling the audio category on iOS and macOS.\n*\n* The variable can be set to the following values:\n*\n* - \"ambient\": Use the AVAudioSessionCategoryAmbient audio category, will be\n*   muted by the phone mute switch (default)\n* - \"playback\": Use the AVAudioSessionCategoryPlayback category.\n*\n* For more information, see Apple's documentation:\n* https://developer.apple.com/library/content/documentation/Audio/Conceptual/AudioSessionProgrammingGuide/AudioSessionCategoriesandModes/AudioSessionCategoriesandModes.html\n*\n* This hint should be set before an audio device is opened.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling the default audio channel count.
*
* If the application doesn't specify the audio channel count when opening the
* device, this hint can be used to specify a default channel count that will
* be used. This defaults to "1" for recording and "2" for playback devices.
*
* This hint should be set before an audio device is opened.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_AUDIO_CHANNELS: Hint = Hint { name: "SDL_HINT_AUDIO_CHANNELS", value: "SDL_AUDIO_CHANNELS", doc: "/**\n* A variable controlling the default audio channel count.\n*\n* If the application doesn't specify the audio channel count when opening the\n* device, this hint can be used to specify a default channel count that will\n* be used. This defaults to \"1\" for recording and \"2\" for playback devices.\n*\n* This hint should be set before an audio device is opened.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* Specify an application icon name for an audio device.
*
* Some audio backends (such as Pulseaudio and Pipewire) allow you to set an
* XDG icon name for your application. Among other things, this icon might
* show up in a system control panel that lets the user adjust the volume on
* specific audio streams instead of using one giant master volume slider.
* Note that this is unrelated to the icon used by the windowing system, which
* may be set with SDL_SetWindowIcon (or via desktop file on Wayland).
*
* Setting this to "" or leaving it unset will have SDL use a reasonable
* default, "applications-games", which is likely to be installed. See
* https://specifications.freedesktop.org/icon-theme-spec/icon-theme-spec-latest.html
* and
* https://specifications.freedesktop.org/icon-naming-spec/icon-naming-spec-latest.html
* for the relevant XDG icon specs.
*
* This hint should be set before an audio device is opened.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_AUDIO_DEVICE_APP_ICON_NAME: Hint = Hint { name: "SDL_HINT_AUDIO_DEVICE_APP_ICON_NAME", value: "SDL_AUDIO_DEVICE_APP_ICON_NAME", doc: "/**\n* Specify an application icon name for an audio device.\n*\n* Some audio backends (such as Pulseaudio and Pipewire) allow you to set an\n* XDG icon name for your application. Among other things, this icon might\n* show up in a system control panel that lets the user adjust the volume on\n* specific audio streams instead of using one giant master volume slider.\n* Note that this is unrelated to the icon used by the windowing system, which\n* may be set with SDL_SetWindowIcon (or via desktop file on Wayland).\n*\n* Setting this to \"\" or leaving it unset will have SDL use a reasonable\n* default, \"applications-games\", which is likely to be installed. See\n* https://specifications.freedesktop.org/icon-theme-spec/icon-theme-spec-latest.html\n* and\n* https://specifications.freedesktop.org/icon-naming-spec/icon-naming-spec-latest.html\n* for the relevant XDG icon specs.\n*\n* This hint should be set before an audio device is opened.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling device buffer size.
*
* This hint is an integer > 0, that represents the size of the device's
* buffer in sample frames (stereo audio data in 16-bit format is 4 bytes per
* sample frame, for example).
*
* SDL3 generally decides this value on behalf of the app, but if for some
* reason the app needs to dictate this (because they want either lower
* latency or higher throughput AND ARE WILLING TO DEAL WITH what that might
* require of the app), they can specify it.
*
* SDL will try to accommodate this value, but there is no promise you'll get
* the buffer size requested. Many platforms won't honor this request at all,
* or might adjust it.
*
* This hint should be set before an audio device is opened.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_AUDIO_DEVICE_SAMPLE_FRAMES: Hint = Hint { name: "SDL_HINT_AUDIO_DEVICE_SAMPLE_FRAMES", value: "SDL_AUDIO_DEVICE_SAMPLE_FRAMES", doc: "/**\n* A variable controlling device buffer size.\n*\n* This hint is an integer > 0, that represents the size of the device's\n* buffer in sample frames (stereo audio data in 16-bit format is 4 bytes per\n* sample frame, for example).\n*\n* SDL3 generally decides this value on behalf of the app, but if for some\n* reason the app needs to dictate this (because they want either lower\n* latency or higher throughput AND ARE WILLING TO DEAL WITH what that might\n* require of the app), they can specify it.\n*\n* SDL will try to accommodate this value, but there is no promise you'll get\n* the buffer size requested. Many platforms won't honor this request at all,\n* or might adjust it.\n*\n* This hint should be set before an audio device is opened.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* Specify an audio stream name for an audio device.
*
* Some audio backends (such as PulseAudio) allow you to describe your audio
* stream. Among other things, this description might show up in a system
* control panel that lets the user adjust the volume on specific audio
* streams instead of using one giant master volume slider.
*
* This hints lets you transmit that information to the OS. The contents of
* this hint are used while opening an audio device. You should use a string
* that describes your what your program is playing ("audio stream" is
* probably sufficient in many cases, but this could be useful for something
* like "team chat" if you have a headset playing VoIP audio separately).
*
* Setting this to "" or leaving it unset will have SDL use a reasonable
* default: "audio stream" or something similar.
*
* Note that while this talks about audio streams, this is an OS-level
* concept, so it applies to a physical audio device in this case, and not an
* SDL_AudioStream, nor an SDL logical audio device.
*
* This hint should be set before an audio device is opened.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_AUDIO_DEVICE_STREAM_NAME: Hint = Hint { name: "SDL_HINT_AUDIO_DEVICE_STREAM_NAME", value: "SDL_AUDIO_DEVICE_STREAM_NAME", doc: "/**\n* Specify an audio stream name for an audio device.\n*\n* Some audio backends (such as PulseAudio) allow you to describe your audio\n* stream. Among other things, this description might show up in a system\n* control panel that lets the user adjust the volume on specific audio\n* streams instead of using one giant master volume slider.\n*\n* This hints lets you transmit that information to the OS. The contents of\n* this hint are used while opening an audio device. You should use a string\n* that describes your what your program is playing (\"audio stream\" is\n* probably sufficient in many cases, but this could be useful for something\n* like \"team chat\" if you have a headset playing VoIP audio separately).\n*\n* Setting this to \"\" or leaving it unset will have SDL use a reasonable\n* default: \"audio stream\" or something similar.\n*\n* Note that while this talks about audio streams, this is an OS-level\n* concept, so it applies to a physical audio device in this case, and not an\n* SDL_AudioStream, nor an SDL logical audio device.\n*\n* This hint should be set before an audio device is opened.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* Specify an application role for an audio device.
*
* Some audio backends (such as Pipewire) allow you to describe the role of
* your audio stream. Among other things, this description might show up in a
* system control panel or software for displaying and manipulating media
* playback/recording graphs.
*
* This hints lets you transmit that information to the OS. The contents of
* this hint are used while opening an audio device. You should use a string
* that describes your what your program is playing (Game, Music, Movie,
* etc...).
*
* Setting this to "" or leaving it unset will have SDL use a reasonable
* default: "Game" or something similar.
*
* Note that while this talks about audio streams, this is an OS-level
* concept, so it applies to a physical audio device in this case, and not an
* SDL_AudioStream, nor an SDL logical audio device.
*
* For Windows WASAPI audio, the following roles are supported, and map to
* `AUDIO_STREAM_CATEGORY`:
*
* - "Other" (default)
* - "Communications" - Real-time communications, such as VOIP or chat
* - "Game" - Game audio
* - "GameChat" - Game chat audio, similar to "Communications" except that
*   this will not attenuate other audio streams
* - "Movie" - Music or sound with dialog
* - "Media" - Music or sound without dialog
*
* Android's AAudio target supports this hint as of SDL 3.4.4. Android does
* not support the exact same options as WASAPI, but for portability, will
* attempt to map these same strings to the `aaudio_usage_t` constants. For
* example, "Movie" and "Media" will both map to `AAUDIO_USAGE_MEDIA`, etc.
*
* If your application applies its own echo cancellation, gain control, and
* noise reduction it should also set SDL_HINT_AUDIO_DEVICE_RAW_STREAM.
*
* This hint should be set before an audio device is opened.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_AUDIO_DEVICE_STREAM_ROLE: Hint = Hint { name: "SDL_HINT_AUDIO_DEVICE_STREAM_ROLE", value: "SDL_AUDIO_DEVICE_STREAM_ROLE", doc: "/**\n* Specify an application role for an audio device.\n*\n* Some audio backends (such as Pipewire) allow you to describe the role of\n* your audio stream. Among other things, this description might show up in a\n* system control panel or software for displaying and manipulating media\n* playback/recording graphs.\n*\n* This hints lets you transmit that information to the OS. The contents of\n* this hint are used while opening an audio device. You should use a string\n* that describes your what your program is playing (Game, Music, Movie,\n* etc...).\n*\n* Setting this to \"\" or leaving it unset will have SDL use a reasonable\n* default: \"Game\" or something similar.\n*\n* Note that while this talks about audio streams, this is an OS-level\n* concept, so it applies to a physical audio device in this case, and not an\n* SDL_AudioStream, nor an SDL logical audio device.\n*\n* For Windows WASAPI audio, the following roles are supported, and map to\n* `AUDIO_STREAM_CATEGORY`:\n*\n* - \"Other\" (default)\n* - \"Communications\" - Real-time communications, such as VOIP or chat\n* - \"Game\" - Game audio\n* - \"GameChat\" - Game chat audio, similar to \"Communications\" except that\n*   this will not attenuate other audio streams\n* - \"Movie\" - Music or sound with dialog\n* - \"Media\" - Music or sound without dialog\n*\n* Android's AAudio target supports this hint as of SDL 3.4.4. Android does\n* not support the exact same options as WASAPI, but for portability, will\n* attempt to map these same strings to the `aaudio_usage_t` constants. For\n* example, \"Movie\" and \"Media\" will both map to `AAUDIO_USAGE_MEDIA`, etc.\n*\n* If your application applies its own echo cancellation, gain control, and\n* noise reduction it should also set SDL_HINT_AUDIO_DEVICE_RAW_STREAM.\n*\n* This hint should be set before an audio device is opened.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* Specify whether this audio device should do audio processing.
*
* Some operating systems perform echo cancellation, gain control, and noise
* reduction as needed. If your application already handles these, you can set
* this hint to prevent the OS from doing additional audio processing.
*
* This corresponds to the WASAPI audio option `AUDCLNT_STREAMOPTIONS_RAW`.
*
* The variable can be set to the following values:
*
* - "0": audio processing can be done by the OS. (default)
* - "1": audio processing is done by the application.
*
* This hint should be set before an audio device is opened.
*
* \since This hint is available since SDL 3.4.0.
*/
pub const SDL_HINT_AUDIO_DEVICE_RAW_STREAM: Hint = Hint { name: "SDL_HINT_AUDIO_DEVICE_RAW_STREAM", value: "SDL_AUDIO_DEVICE_RAW_STREAM", doc: "/**\n* Specify whether this audio device should do audio processing.\n*\n* Some operating systems perform echo cancellation, gain control, and noise\n* reduction as needed. If your application already handles these, you can set\n* this hint to prevent the OS from doing additional audio processing.\n*\n* This corresponds to the WASAPI audio option `AUDCLNT_STREAMOPTIONS_RAW`.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": audio processing can be done by the OS. (default)\n* - \"1\": audio processing is done by the application.\n*\n* This hint should be set before an audio device is opened.\n*\n* \\since This hint is available since SDL 3.4.0.\n*/\n" };
/**
* Specify the input file when recording audio using the disk audio driver.
*
* This defaults to "sdlaudio-in.raw"
*
* This hint should be set before an audio device is opened.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_AUDIO_DISK_INPUT_FILE: Hint = Hint { name: "SDL_HINT_AUDIO_DISK_INPUT_FILE", value: "SDL_AUDIO_DISK_INPUT_FILE", doc: "/**\n* Specify the input file when recording audio using the disk audio driver.\n*\n* This defaults to \"sdlaudio-in.raw\"\n*\n* This hint should be set before an audio device is opened.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* Specify the output file when playing audio using the disk audio driver.
*
* This defaults to "sdlaudio.raw"
*
* This hint should be set before an audio device is opened.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_AUDIO_DISK_OUTPUT_FILE: Hint = Hint { name: "SDL_HINT_AUDIO_DISK_OUTPUT_FILE", value: "SDL_AUDIO_DISK_OUTPUT_FILE", doc: "/**\n* Specify the output file when playing audio using the disk audio driver.\n*\n* This defaults to \"sdlaudio.raw\"\n*\n* This hint should be set before an audio device is opened.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling the audio rate when using the disk audio driver.
*
* The disk audio driver normally simulates real-time for the audio rate that
* was specified, but you can use this variable to adjust this rate higher or
* lower down to 0. The default value is "1.0".
*
* This hint should be set before an audio device is opened.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_AUDIO_DISK_TIMESCALE: Hint = Hint { name: "SDL_HINT_AUDIO_DISK_TIMESCALE", value: "SDL_AUDIO_DISK_TIMESCALE", doc: "/**\n* A variable controlling the audio rate when using the disk audio driver.\n*\n* The disk audio driver normally simulates real-time for the audio rate that\n* was specified, but you can use this variable to adjust this rate higher or\n* lower down to 0. The default value is \"1.0\".\n*\n* This hint should be set before an audio device is opened.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable that specifies an audio backend to use.
*
* By default, SDL will try all available audio backends in a reasonable order
* until it finds one that can work, but this hint allows the app or user to
* force a specific driver, such as "pipewire" if, say, you are on PulseAudio
* but want to try talking to the lower level instead.
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_AUDIO_DRIVER: Hint = Hint { name: "SDL_HINT_AUDIO_DRIVER", value: "SDL_AUDIO_DRIVER", doc: "/**\n* A variable that specifies an audio backend to use.\n*\n* By default, SDL will try all available audio backends in a reasonable order\n* until it finds one that can work, but this hint allows the app or user to\n* force a specific driver, such as \"pipewire\" if, say, you are on PulseAudio\n* but want to try talking to the lower level instead.\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling the audio rate when using the dummy audio driver.
*
* The dummy audio driver normally simulates real-time for the audio rate that
* was specified, but you can use this variable to adjust this rate higher or
* lower down to 0. The default value is "1.0".
*
* This hint should be set before an audio device is opened.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_AUDIO_DUMMY_TIMESCALE: Hint = Hint { name: "SDL_HINT_AUDIO_DUMMY_TIMESCALE", value: "SDL_AUDIO_DUMMY_TIMESCALE", doc: "/**\n* A variable controlling the audio rate when using the dummy audio driver.\n*\n* The dummy audio driver normally simulates real-time for the audio rate that\n* was specified, but you can use this variable to adjust this rate higher or\n* lower down to 0. The default value is \"1.0\".\n*\n* This hint should be set before an audio device is opened.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling the default audio format.
*
* If the application doesn't specify the audio format when opening the
* device, this hint can be used to specify a default format that will be
* used.
*
* The variable can be set to the following values:
*
* - "U8": Unsigned 8-bit audio
* - "S8": Signed 8-bit audio
* - "S16LE": Signed 16-bit little-endian audio
* - "S16BE": Signed 16-bit big-endian audio
* - "S16": Signed 16-bit native-endian audio (default)
* - "S32LE": Signed 32-bit little-endian audio
* - "S32BE": Signed 32-bit big-endian audio
* - "S32": Signed 32-bit native-endian audio
* - "F32LE": Floating point little-endian audio
* - "F32BE": Floating point big-endian audio
* - "F32": Floating point native-endian audio
*
* This hint should be set before an audio device is opened.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_AUDIO_FORMAT: Hint = Hint { name: "SDL_HINT_AUDIO_FORMAT", value: "SDL_AUDIO_FORMAT", doc: "/**\n* A variable controlling the default audio format.\n*\n* If the application doesn't specify the audio format when opening the\n* device, this hint can be used to specify a default format that will be\n* used.\n*\n* The variable can be set to the following values:\n*\n* - \"U8\": Unsigned 8-bit audio\n* - \"S8\": Signed 8-bit audio\n* - \"S16LE\": Signed 16-bit little-endian audio\n* - \"S16BE\": Signed 16-bit big-endian audio\n* - \"S16\": Signed 16-bit native-endian audio (default)\n* - \"S32LE\": Signed 32-bit little-endian audio\n* - \"S32BE\": Signed 32-bit big-endian audio\n* - \"S32\": Signed 32-bit native-endian audio\n* - \"F32LE\": Floating point little-endian audio\n* - \"F32BE\": Floating point big-endian audio\n* - \"F32\": Floating point native-endian audio\n*\n* This hint should be set before an audio device is opened.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling the default audio frequency.
*
* If the application doesn't specify the audio frequency when opening the
* device, this hint can be used to specify a default frequency that will be
* used. This defaults to "44100".
*
* This hint should be set before an audio device is opened.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_AUDIO_FREQUENCY: Hint = Hint { name: "SDL_HINT_AUDIO_FREQUENCY", value: "SDL_AUDIO_FREQUENCY", doc: "/**\n* A variable controlling the default audio frequency.\n*\n* If the application doesn't specify the audio frequency when opening the\n* device, this hint can be used to specify a default frequency that will be\n* used. This defaults to \"44100\".\n*\n* This hint should be set before an audio device is opened.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable that causes SDL to not ignore audio "monitors".
*
* This is currently only used by the PulseAudio driver.
*
* By default, SDL ignores audio devices that aren't associated with physical
* hardware. Changing this hint to "1" will expose anything SDL sees that
* appears to be an audio source or sink. This will add "devices" to the list
* that the user probably doesn't want or need, but it can be useful in
* scenarios where you want to hook up SDL to some sort of virtual device,
* etc.
*
* The variable can be set to the following values:
*
* - "0": Audio monitor devices will be ignored. (default)
* - "1": Audio monitor devices will show up in the device list.
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_AUDIO_INCLUDE_MONITORS: Hint = Hint { name: "SDL_HINT_AUDIO_INCLUDE_MONITORS", value: "SDL_AUDIO_INCLUDE_MONITORS", doc: "/**\n* A variable that causes SDL to not ignore audio \"monitors\".\n*\n* This is currently only used by the PulseAudio driver.\n*\n* By default, SDL ignores audio devices that aren't associated with physical\n* hardware. Changing this hint to \"1\" will expose anything SDL sees that\n* appears to be an audio source or sink. This will add \"devices\" to the list\n* that the user probably doesn't want or need, but it can be useful in\n* scenarios where you want to hook up SDL to some sort of virtual device,\n* etc.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Audio monitor devices will be ignored. (default)\n* - \"1\": Audio monitor devices will show up in the device list.\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether SDL updates joystick state when getting
* input events.
*
* The variable can be set to the following values:
*
* - "0": You'll call SDL_UpdateJoysticks() manually.
* - "1": SDL will automatically call SDL_UpdateJoysticks(). (default)
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_AUTO_UPDATE_JOYSTICKS: Hint = Hint { name: "SDL_HINT_AUTO_UPDATE_JOYSTICKS", value: "SDL_AUTO_UPDATE_JOYSTICKS", doc: "/**\n* A variable controlling whether SDL updates joystick state when getting\n* input events.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": You'll call SDL_UpdateJoysticks() manually.\n* - \"1\": SDL will automatically call SDL_UpdateJoysticks(). (default)\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether SDL updates sensor state when getting input
* events.
*
* The variable can be set to the following values:
*
* - "0": You'll call SDL_UpdateSensors() manually.
* - "1": SDL will automatically call SDL_UpdateSensors(). (default)
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_AUTO_UPDATE_SENSORS: Hint = Hint { name: "SDL_HINT_AUTO_UPDATE_SENSORS", value: "SDL_AUTO_UPDATE_SENSORS", doc: "/**\n* A variable controlling whether SDL updates sensor state when getting input\n* events.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": You'll call SDL_UpdateSensors() manually.\n* - \"1\": SDL will automatically call SDL_UpdateSensors(). (default)\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* Prevent SDL from using version 4 of the bitmap header when saving BMPs.
*
* The bitmap header version 4 is required for proper alpha channel support
* and SDL will use it when required. Should this not be desired, this hint
* can force the use of the 40 byte header version which is supported
* everywhere.
*
* The variable can be set to the following values:
*
* - "0": Surfaces with a colorkey or an alpha channel are saved to a 32-bit
*   BMP file with an alpha mask. SDL will use the bitmap header version 4 and
*   set the alpha mask accordingly. (default)
* - "1": Surfaces with a colorkey or an alpha channel are saved to a 32-bit
*   BMP file without an alpha mask. The alpha channel data will be in the
*   file, but applications are going to ignore it.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_BMP_SAVE_LEGACY_FORMAT: Hint = Hint { name: "SDL_HINT_BMP_SAVE_LEGACY_FORMAT", value: "SDL_BMP_SAVE_LEGACY_FORMAT", doc: "/**\n* Prevent SDL from using version 4 of the bitmap header when saving BMPs.\n*\n* The bitmap header version 4 is required for proper alpha channel support\n* and SDL will use it when required. Should this not be desired, this hint\n* can force the use of the 40 byte header version which is supported\n* everywhere.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Surfaces with a colorkey or an alpha channel are saved to a 32-bit\n*   BMP file with an alpha mask. SDL will use the bitmap header version 4 and\n*   set the alpha mask accordingly. (default)\n* - \"1\": Surfaces with a colorkey or an alpha channel are saved to a 32-bit\n*   BMP file without an alpha mask. The alpha channel data will be in the\n*   file, but applications are going to ignore it.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable that decides what camera backend to use.
*
* By default, SDL will try all available camera backends in a reasonable
* order until it finds one that can work, but this hint allows the app or
* user to force a specific target, such as "directshow" if, say, you are on
* Windows Media Foundations but want to try DirectShow instead.
*
* The default value is unset, in which case SDL will try to figure out the
* best camera backend on your behalf. This hint needs to be set before
* SDL_Init() is called to be useful.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_CAMERA_DRIVER: Hint = Hint { name: "SDL_HINT_CAMERA_DRIVER", value: "SDL_CAMERA_DRIVER", doc: "/**\n* A variable that decides what camera backend to use.\n*\n* By default, SDL will try all available camera backends in a reasonable\n* order until it finds one that can work, but this hint allows the app or\n* user to force a specific target, such as \"directshow\" if, say, you are on\n* Windows Media Foundations but want to try DirectShow instead.\n*\n* The default value is unset, in which case SDL will try to figure out the\n* best camera backend on your behalf. This hint needs to be set before\n* SDL_Init() is called to be useful.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable that limits what CPU features are available.
*
* By default, SDL marks all features the current CPU supports as available.
* This hint allows the enabled features to be limited to a subset.
*
* When the hint is unset, or empty, SDL will enable all detected CPU
* features.
*
* The variable can be set to a comma separated list containing the following
* items:
*
* - "all"
* - "altivec"
* - "sse"
* - "sse2"
* - "sse3"
* - "sse41"
* - "sse42"
* - "avx"
* - "avx2"
* - "avx512f"
* - "arm-simd"
* - "neon"
* - "lsx"
* - "lasx"
*
* The items can be prefixed by '+'/'-' to add/remove features.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_CPU_FEATURE_MASK: Hint = Hint { name: "SDL_HINT_CPU_FEATURE_MASK", value: "SDL_CPU_FEATURE_MASK", doc: "/**\n* A variable that limits what CPU features are available.\n*\n* By default, SDL marks all features the current CPU supports as available.\n* This hint allows the enabled features to be limited to a subset.\n*\n* When the hint is unset, or empty, SDL will enable all detected CPU\n* features.\n*\n* The variable can be set to a comma separated list containing the following\n* items:\n*\n* - \"all\"\n* - \"altivec\"\n* - \"sse\"\n* - \"sse2\"\n* - \"sse3\"\n* - \"sse41\"\n* - \"sse42\"\n* - \"avx\"\n* - \"avx2\"\n* - \"avx512f\"\n* - \"arm-simd\"\n* - \"neon\"\n* - \"lsx\"\n* - \"lasx\"\n*\n* The items can be prefixed by '+'/'-' to add/remove features.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether DirectInput should be used for controllers.
*
* The variable can be set to the following values:
*
* - "0": Disable DirectInput detection.
* - "1": Enable DirectInput detection. (default)
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_DIRECTINPUT: Hint = Hint { name: "SDL_HINT_JOYSTICK_DIRECTINPUT", value: "SDL_JOYSTICK_DIRECTINPUT", doc: "/**\n* A variable controlling whether DirectInput should be used for controllers.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Disable DirectInput detection.\n* - \"1\": Enable DirectInput detection. (default)\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable that specifies a dialog backend to use.
*
* By default, SDL will try all available dialog backends in a reasonable
* order until it finds one that can work, but this hint allows the app or
* user to force a specific target.
*
* If the specified target does not exist or is not available, the
* dialog-related function calls will fail.
*
* This hint currently only applies to platforms using the generic "Unix"
* dialog implementation, but may be extended to more platforms in the future.
* Note that some Unix and Unix-like platforms have their own implementation,
* such as macOS and Haiku.
*
* The variable can be set to the following values:
*
* - NULL: Select automatically (default, all platforms)
* - "portal": Use XDG Portals through DBus (Unix only)
* - "zenity": Use the Zenity program (Unix only)
*
* More options may be added in the future.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_FILE_DIALOG_DRIVER: Hint = Hint { name: "SDL_HINT_FILE_DIALOG_DRIVER", value: "SDL_FILE_DIALOG_DRIVER", doc: "/**\n* A variable that specifies a dialog backend to use.\n*\n* By default, SDL will try all available dialog backends in a reasonable\n* order until it finds one that can work, but this hint allows the app or\n* user to force a specific target.\n*\n* If the specified target does not exist or is not available, the\n* dialog-related function calls will fail.\n*\n* This hint currently only applies to platforms using the generic \"Unix\"\n* dialog implementation, but may be extended to more platforms in the future.\n* Note that some Unix and Unix-like platforms have their own implementation,\n* such as macOS and Haiku.\n*\n* The variable can be set to the following values:\n*\n* - NULL: Select automatically (default, all platforms)\n* - \"portal\": Use XDG Portals through DBus (Unix only)\n* - \"zenity\": Use the Zenity program (Unix only)\n*\n* More options may be added in the future.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* Override for SDL_GetDisplayUsableBounds().
*
* If set, this hint will override the expected results for
* SDL_GetDisplayUsableBounds() for display index 0. Generally you don't want
* to do this, but this allows an embedded system to request that some of the
* screen be reserved for other uses when paired with a well-behaved
* application.
*
* The contents of this hint must be 4 comma-separated integers, the first is
* the bounds x, then y, width and height, in that order.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_DISPLAY_USABLE_BOUNDS: Hint = Hint { name: "SDL_HINT_DISPLAY_USABLE_BOUNDS", value: "SDL_DISPLAY_USABLE_BOUNDS", doc: "/**\n* Override for SDL_GetDisplayUsableBounds().\n*\n* If set, this hint will override the expected results for\n* SDL_GetDisplayUsableBounds() for display index 0. Generally you don't want\n* to do this, but this allows an embedded system to request that some of the\n* screen be reserved for other uses when paired with a well-behaved\n* application.\n*\n* The contents of this hint must be 4 comma-separated integers, the first is\n* the bounds x, then y, width and height, in that order.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable that enables a fast framebuffer path on DOS.
*
* When set to "1", SDL_UpdateWindowSurface() copies the system-RAM surface
* directly to VRAM and skips software cursor compositing and vsync.
*
* The variable can be set to the following values:
*
* - "0": Use the normal path with cursor compositing and vsync. (default)
* - "1": Use the fast direct-to-VRAM path when available.
*
* This hint must be set before the first call to SDL_GetWindowSurface().
*
* \since This hint is available since SDL 3.6.0.
*/
pub const SDL_HINT_DOS_ALLOW_DIRECT_FRAMEBUFFER: Hint = Hint { name: "SDL_HINT_DOS_ALLOW_DIRECT_FRAMEBUFFER", value: "SDL_DOS_ALLOW_DIRECT_FRAMEBUFFER", doc: "/**\n* A variable that enables a fast framebuffer path on DOS.\n*\n* When set to \"1\", SDL_UpdateWindowSurface() copies the system-RAM surface\n* directly to VRAM and skips software cursor compositing and vsync.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Use the normal path with cursor compositing and vsync. (default)\n* - \"1\": Use the fast direct-to-VRAM path when available.\n*\n* This hint must be set before the first call to SDL_GetWindowSurface().\n*\n* \\since This hint is available since SDL 3.6.0.\n*/\n" };
/**
* Set the level of checking for invalid parameters passed to SDL functions.
*
* The variable can be set to the following values:
*
* - "1": Enable fast parameter error checking, e.g. quick NULL checks, etc.
* - "2": Enable full parameter error checking, e.g. validating objects are
*   the correct type, etc. (default)
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.4.0.
*/
pub const SDL_HINT_INVALID_PARAM_CHECKS: Hint = Hint { name: "SDL_HINT_INVALID_PARAM_CHECKS", value: "SDL_INVALID_PARAM_CHECKS", doc: "/**\n* Set the level of checking for invalid parameters passed to SDL functions.\n*\n* The variable can be set to the following values:\n*\n* - \"1\": Enable fast parameter error checking, e.g. quick NULL checks, etc.\n* - \"2\": Enable full parameter error checking, e.g. validating objects are\n*   the correct type, etc. (default)\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.4.0.\n*/\n" };
/**
* Disable giving back control to the browser automatically when running with
* asyncify.
*
* With -s ASYNCIFY, SDL calls emscripten_sleep during operations such as
* refreshing the screen or polling events.
*
* This hint only applies to the emscripten platform.
*
* The variable can be set to the following values:
*
* - "0": Disable emscripten_sleep calls (if you give back browser control
*   manually or use asyncify for other purposes).
* - "1": Enable emscripten_sleep calls. (default)
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_EMSCRIPTEN_ASYNCIFY: Hint = Hint { name: "SDL_HINT_EMSCRIPTEN_ASYNCIFY", value: "SDL_EMSCRIPTEN_ASYNCIFY", doc: "/**\n* Disable giving back control to the browser automatically when running with\n* asyncify.\n*\n* With -s ASYNCIFY, SDL calls emscripten_sleep during operations such as\n* refreshing the screen or polling events.\n*\n* This hint only applies to the emscripten platform.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Disable emscripten_sleep calls (if you give back browser control\n*   manually or use asyncify for other purposes).\n* - \"1\": Enable emscripten_sleep calls. (default)\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* Specify the CSS selector used for the "default" window/canvas.
*
* This hint only applies to the emscripten platform.
*
* This hint should be set before creating a window.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_EMSCRIPTEN_CANVAS_SELECTOR: Hint = Hint { name: "SDL_HINT_EMSCRIPTEN_CANVAS_SELECTOR", value: "SDL_EMSCRIPTEN_CANVAS_SELECTOR", doc: "/**\n* Specify the CSS selector used for the \"default\" window/canvas.\n*\n* This hint only applies to the emscripten platform.\n*\n* This hint should be set before creating a window.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* Override the binding element for keyboard inputs for Emscripten builds.
*
* This hint only applies to the emscripten platform.
*
* The variable can be one of:
*
* - "#window": the javascript window object
* - "#document": the javascript document object
* - "#screen": the javascript window.screen object
* - "#canvas": the WebGL canvas element
* - "#none": Don't bind anything at all
* - any other string without a leading # sign applies to the element on the
*   page with that ID.
*
* This hint should be set before creating a window.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_EMSCRIPTEN_KEYBOARD_ELEMENT: Hint = Hint { name: "SDL_HINT_EMSCRIPTEN_KEYBOARD_ELEMENT", value: "SDL_EMSCRIPTEN_KEYBOARD_ELEMENT", doc: "/**\n* Override the binding element for keyboard inputs for Emscripten builds.\n*\n* This hint only applies to the emscripten platform.\n*\n* The variable can be one of:\n*\n* - \"#window\": the javascript window object\n* - \"#document\": the javascript document object\n* - \"#screen\": the javascript window.screen object\n* - \"#canvas\": the WebGL canvas element\n* - \"#none\": Don't bind anything at all\n* - any other string without a leading # sign applies to the element on the\n*   page with that ID.\n*\n* This hint should be set before creating a window.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable that controls whether the on-screen keyboard should be shown
* when text input is active.
*
* The variable can be set to the following values:
*
* - "auto": The on-screen keyboard will be shown if there is no physical
*   keyboard attached. (default)
* - "0": Do not show the on-screen keyboard.
* - "1": Show the on-screen keyboard, if available.
*
* This hint must be set before SDL_StartTextInput() is called
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_ENABLE_SCREEN_KEYBOARD: Hint = Hint { name: "SDL_HINT_ENABLE_SCREEN_KEYBOARD", value: "SDL_ENABLE_SCREEN_KEYBOARD", doc: "/**\n* A variable that controls whether the on-screen keyboard should be shown\n* when text input is active.\n*\n* The variable can be set to the following values:\n*\n* - \"auto\": The on-screen keyboard will be shown if there is no physical\n*   keyboard attached. (default)\n* - \"0\": Do not show the on-screen keyboard.\n* - \"1\": Show the on-screen keyboard, if available.\n*\n* This hint must be set before SDL_StartTextInput() is called\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable containing a list of evdev devices to use if udev is not
* available.
*
* The list of devices is in the form:
*
* deviceclass:path[,deviceclass:path[,...]]
*
* where device class is an integer representing the SDL_UDEV_deviceclass and
* path is the full path to the event device.
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_EVDEV_DEVICES: Hint = Hint { name: "SDL_HINT_EVDEV_DEVICES", value: "SDL_EVDEV_DEVICES", doc: "/**\n* A variable containing a list of evdev devices to use if udev is not\n* available.\n*\n* The list of devices is in the form:\n*\n* deviceclass:path[,deviceclass:path[,...]]\n*\n* where device class is an integer representing the SDL_UDEV_deviceclass and\n* path is the full path to the event device.\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling verbosity of the logging of SDL events pushed onto
* the internal queue.
*
* The variable can be set to the following values, from least to most
* verbose:
*
* - "0": Don't log any events. (default)
* - "1": Log most events (other than the really spammy ones).
* - "2": Include mouse and finger motion events.
*
* This is generally meant to be used to debug SDL itself, but can be useful
* for application developers that need better visibility into what is going
* on in the event queue. Logged events are sent through SDL_Log(), which
* means by default they appear on stdout on most platforms or maybe
* OutputDebugString() on Windows, and can be funneled by the app with
* SDL_SetLogOutputFunction(), etc.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_EVENT_LOGGING: Hint = Hint { name: "SDL_HINT_EVENT_LOGGING", value: "SDL_EVENT_LOGGING", doc: "/**\n* A variable controlling verbosity of the logging of SDL events pushed onto\n* the internal queue.\n*\n* The variable can be set to the following values, from least to most\n* verbose:\n*\n* - \"0\": Don't log any events. (default)\n* - \"1\": Log most events (other than the really spammy ones).\n* - \"2\": Include mouse and finger motion events.\n*\n* This is generally meant to be used to debug SDL itself, but can be useful\n* for application developers that need better visibility into what is going\n* on in the event queue. Logged events are sent through SDL_Log(), which\n* means by default they appear on stdout on most platforms or maybe\n* OutputDebugString() on Windows, and can be funneled by the app with\n* SDL_SetLogOutputFunction(), etc.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether raising the window should be done more
* forcefully.
*
* The variable can be set to the following values:
*
* - "0": Honor the OS policy for raising windows. (default)
* - "1": Force the window to be raised, overriding any OS policy.
*
* At present, this is only an issue under MS Windows, which makes it nearly
* impossible to programmatically move a window to the foreground, for
* "security" reasons. See http://stackoverflow.com/a/34414846 for a
* discussion.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_FORCE_RAISEWINDOW: Hint = Hint { name: "SDL_HINT_FORCE_RAISEWINDOW", value: "SDL_FORCE_RAISEWINDOW", doc: "/**\n* A variable controlling whether raising the window should be done more\n* forcefully.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Honor the OS policy for raising windows. (default)\n* - \"1\": Force the window to be raised, overriding any OS policy.\n*\n* At present, this is only an issue under MS Windows, which makes it nearly\n* impossible to programmatically move a window to the foreground, for\n* \"security\" reasons. See http://stackoverflow.com/a/34414846 for a\n* discussion.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling how 3D acceleration is used to accelerate the SDL
* screen surface.
*
* SDL can try to accelerate the SDL screen surface by using streaming
* textures with a 3D rendering engine. This variable controls whether and how
* this is done.
*
* The variable can be set to the following values:
*
* - "0": Disable 3D acceleration
* - "1": Enable 3D acceleration, using the default renderer. (default)
* - "X": Enable 3D acceleration, using X where X is one of the valid
*   rendering drivers. (e.g. "direct3d", "opengl", etc.)
*
* This hint should be set before calling SDL_GetWindowSurface()
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_FRAMEBUFFER_ACCELERATION: Hint = Hint { name: "SDL_HINT_FRAMEBUFFER_ACCELERATION", value: "SDL_FRAMEBUFFER_ACCELERATION", doc: "/**\n* A variable controlling how 3D acceleration is used to accelerate the SDL\n* screen surface.\n*\n* SDL can try to accelerate the SDL screen surface by using streaming\n* textures with a 3D rendering engine. This variable controls whether and how\n* this is done.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Disable 3D acceleration\n* - \"1\": Enable 3D acceleration, using the default renderer. (default)\n* - \"X\": Enable 3D acceleration, using X where X is one of the valid\n*   rendering drivers. (e.g. \"direct3d\", \"opengl\", etc.)\n*\n* This hint should be set before calling SDL_GetWindowSurface()\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable that lets you manually hint extra gamecontroller db entries.
*
* The variable should be newline delimited rows of gamecontroller config
* data, see SDL_gamepad.h
*
* You can update mappings after SDL is initialized with
* SDL_GetGamepadMappingForGUID() and SDL_AddGamepadMapping()
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_GAMECONTROLLERCONFIG: Hint = Hint { name: "SDL_HINT_GAMECONTROLLERCONFIG", value: "SDL_GAMECONTROLLERCONFIG", doc: "/**\n* A variable that lets you manually hint extra gamecontroller db entries.\n*\n* The variable should be newline delimited rows of gamecontroller config\n* data, see SDL_gamepad.h\n*\n* You can update mappings after SDL is initialized with\n* SDL_GetGamepadMappingForGUID() and SDL_AddGamepadMapping()\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable that lets you provide a file with extra gamecontroller db
* entries.
*
* The file should contain lines of gamecontroller config data, see
* SDL_gamepad.h
*
* You can update mappings after SDL is initialized with
* SDL_GetGamepadMappingForGUID() and SDL_AddGamepadMapping()
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_GAMECONTROLLERCONFIG_FILE: Hint = Hint { name: "SDL_HINT_GAMECONTROLLERCONFIG_FILE", value: "SDL_GAMECONTROLLERCONFIG_FILE", doc: "/**\n* A variable that lets you provide a file with extra gamecontroller db\n* entries.\n*\n* The file should contain lines of gamecontroller config data, see\n* SDL_gamepad.h\n*\n* You can update mappings after SDL is initialized with\n* SDL_GetGamepadMappingForGUID() and SDL_AddGamepadMapping()\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable that overrides the automatic controller type detection.
*
* The variable should be comma separated entries, in the form: VID/PID=type
*
* The VID and PID should be hexadecimal with exactly 4 digits, e.g. 0x00fd
*
* This hint affects what low level protocol is used with the HIDAPI driver.
*
* The variable can be set to the following values:
*
* - "Xbox360"
* - "XboxOne"
* - "PS3"
* - "PS4"
* - "PS5"
* - "SwitchPro"
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_GAMECONTROLLERTYPE: Hint = Hint { name: "SDL_HINT_GAMECONTROLLERTYPE", value: "SDL_GAMECONTROLLERTYPE", doc: "/**\n* A variable that overrides the automatic controller type detection.\n*\n* The variable should be comma separated entries, in the form: VID/PID=type\n*\n* The VID and PID should be hexadecimal with exactly 4 digits, e.g. 0x00fd\n*\n* This hint affects what low level protocol is used with the HIDAPI driver.\n*\n* The variable can be set to the following values:\n*\n* - \"Xbox360\"\n* - \"XboxOne\"\n* - \"PS3\"\n* - \"PS4\"\n* - \"PS5\"\n* - \"SwitchPro\"\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable containing a list of devices to skip when scanning for game
* controllers.
*
* The format of the string is a comma separated list of USB VID/PID pairs in
* hexadecimal form, e.g.
*
* 0xAAAA/0xBBBB,0xCCCC/0xDDDD
*
* The variable can also take the form of "@file", in which case the named
* file will be loaded and interpreted as the value of the variable.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_GAMECONTROLLER_IGNORE_DEVICES: Hint = Hint { name: "SDL_HINT_GAMECONTROLLER_IGNORE_DEVICES", value: "SDL_GAMECONTROLLER_IGNORE_DEVICES", doc: "/**\n* A variable containing a list of devices to skip when scanning for game\n* controllers.\n*\n* The format of the string is a comma separated list of USB VID/PID pairs in\n* hexadecimal form, e.g.\n*\n* 0xAAAA/0xBBBB,0xCCCC/0xDDDD\n*\n* The variable can also take the form of \"@file\", in which case the named\n* file will be loaded and interpreted as the value of the variable.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* If set, all devices will be skipped when scanning for game controllers
* except for the ones listed in this variable.
*
* The format of the string is a comma separated list of USB VID/PID pairs in
* hexadecimal form, e.g.
*
* 0xAAAA/0xBBBB,0xCCCC/0xDDDD
*
* The variable can also take the form of "@file", in which case the named
* file will be loaded and interpreted as the value of the variable.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_GAMECONTROLLER_IGNORE_DEVICES_EXCEPT: Hint = Hint { name: "SDL_HINT_GAMECONTROLLER_IGNORE_DEVICES_EXCEPT", value: "SDL_GAMECONTROLLER_IGNORE_DEVICES_EXCEPT", doc: "/**\n* If set, all devices will be skipped when scanning for game controllers\n* except for the ones listed in this variable.\n*\n* The format of the string is a comma separated list of USB VID/PID pairs in\n* hexadecimal form, e.g.\n*\n* 0xAAAA/0xBBBB,0xCCCC/0xDDDD\n*\n* The variable can also take the form of \"@file\", in which case the named\n* file will be loaded and interpreted as the value of the variable.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable that controls whether the device's built-in accelerometer and
* gyro should be used as sensors for gamepads.
*
* The variable can be set to the following values:
*
* - "0": Sensor fusion is disabled
* - "1": Sensor fusion is enabled for all controllers that lack sensors
*
* Or the variable can be a comma separated list of USB VID/PID pairs in
* hexadecimal form, e.g.
*
* 0xAAAA/0xBBBB,0xCCCC/0xDDDD
*
* The variable can also take the form of "@file", in which case the named
* file will be loaded and interpreted as the value of the variable.
*
* This hint should be set before a gamepad is opened.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_GAMECONTROLLER_SENSOR_FUSION: Hint = Hint { name: "SDL_HINT_GAMECONTROLLER_SENSOR_FUSION", value: "SDL_GAMECONTROLLER_SENSOR_FUSION", doc: "/**\n* A variable that controls whether the device's built-in accelerometer and\n* gyro should be used as sensors for gamepads.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Sensor fusion is disabled\n* - \"1\": Sensor fusion is enabled for all controllers that lack sensors\n*\n* Or the variable can be a comma separated list of USB VID/PID pairs in\n* hexadecimal form, e.g.\n*\n* 0xAAAA/0xBBBB,0xCCCC/0xDDDD\n*\n* The variable can also take the form of \"@file\", in which case the named\n* file will be loaded and interpreted as the value of the variable.\n*\n* This hint should be set before a gamepad is opened.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* This variable sets the default text of the TextInput window on GDK
* platforms.
*
* This hint is available only if SDL_GDK_TEXTINPUT defined.
*
* This hint should be set before calling SDL_StartTextInput()
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_GDK_TEXTINPUT_DEFAULT_TEXT: Hint = Hint { name: "SDL_HINT_GDK_TEXTINPUT_DEFAULT_TEXT", value: "SDL_GDK_TEXTINPUT_DEFAULT_TEXT", doc: "/**\n* This variable sets the default text of the TextInput window on GDK\n* platforms.\n*\n* This hint is available only if SDL_GDK_TEXTINPUT defined.\n*\n* This hint should be set before calling SDL_StartTextInput()\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* This variable sets the description of the TextInput window on GDK
* platforms.
*
* This hint is available only if SDL_GDK_TEXTINPUT defined.
*
* This hint should be set before calling SDL_StartTextInput()
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_GDK_TEXTINPUT_DESCRIPTION: Hint = Hint { name: "SDL_HINT_GDK_TEXTINPUT_DESCRIPTION", value: "SDL_GDK_TEXTINPUT_DESCRIPTION", doc: "/**\n* This variable sets the description of the TextInput window on GDK\n* platforms.\n*\n* This hint is available only if SDL_GDK_TEXTINPUT defined.\n*\n* This hint should be set before calling SDL_StartTextInput()\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* This variable sets the maximum input length of the TextInput window on GDK
* platforms.
*
* The value must be a stringified integer, for example "10" to allow for up
* to 10 characters of text input.
*
* This hint is available only if SDL_GDK_TEXTINPUT defined.
*
* This hint should be set before calling SDL_StartTextInput()
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_GDK_TEXTINPUT_MAX_LENGTH: Hint = Hint { name: "SDL_HINT_GDK_TEXTINPUT_MAX_LENGTH", value: "SDL_GDK_TEXTINPUT_MAX_LENGTH", doc: "/**\n* This variable sets the maximum input length of the TextInput window on GDK\n* platforms.\n*\n* The value must be a stringified integer, for example \"10\" to allow for up\n* to 10 characters of text input.\n*\n* This hint is available only if SDL_GDK_TEXTINPUT defined.\n*\n* This hint should be set before calling SDL_StartTextInput()\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* This variable sets the input scope of the TextInput window on GDK
* platforms.
*
* Set this hint to change the XGameUiTextEntryInputScope value that will be
* passed to the window creation function. The value must be a stringified
* integer, for example "0" for XGameUiTextEntryInputScope::Default.
*
* This hint is available only if SDL_GDK_TEXTINPUT defined.
*
* This hint should be set before calling SDL_StartTextInput()
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_GDK_TEXTINPUT_SCOPE: Hint = Hint { name: "SDL_HINT_GDK_TEXTINPUT_SCOPE", value: "SDL_GDK_TEXTINPUT_SCOPE", doc: "/**\n* This variable sets the input scope of the TextInput window on GDK\n* platforms.\n*\n* Set this hint to change the XGameUiTextEntryInputScope value that will be\n* passed to the window creation function. The value must be a stringified\n* integer, for example \"0\" for XGameUiTextEntryInputScope::Default.\n*\n* This hint is available only if SDL_GDK_TEXTINPUT defined.\n*\n* This hint should be set before calling SDL_StartTextInput()\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* This variable sets the title of the TextInput window on GDK platforms.
*
* This hint is available only if SDL_GDK_TEXTINPUT defined.
*
* This hint should be set before calling SDL_StartTextInput()
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_GDK_TEXTINPUT_TITLE: Hint = Hint { name: "SDL_HINT_GDK_TEXTINPUT_TITLE", value: "SDL_GDK_TEXTINPUT_TITLE", doc: "/**\n* This variable sets the title of the TextInput window on GDK platforms.\n*\n* This hint is available only if SDL_GDK_TEXTINPUT defined.\n*\n* This hint should be set before calling SDL_StartTextInput()\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable to control whether HIDAPI uses libusb for device access.
*
* By default libusb will only be used for a few devices that require direct
* USB access, and this can be controlled with
* SDL_HINT_HIDAPI_LIBUSB_WHITELIST.
*
* The variable can be set to the following values:
*
* - "0": HIDAPI will not use libusb for device access.
* - "1": HIDAPI will use libusb for device access if available. (default)
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_HIDAPI_LIBUSB: Hint = Hint { name: "SDL_HINT_HIDAPI_LIBUSB", value: "SDL_HIDAPI_LIBUSB", doc: "/**\n* A variable to control whether HIDAPI uses libusb for device access.\n*\n* By default libusb will only be used for a few devices that require direct\n* USB access, and this can be controlled with\n* SDL_HINT_HIDAPI_LIBUSB_WHITELIST.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": HIDAPI will not use libusb for device access.\n* - \"1\": HIDAPI will use libusb for device access if available. (default)\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable to control whether HIDAPI uses libusb for GameCube adapters.
*
* The variable can be set to the following values:
*
* - "0": HIDAPI will not use libusb for GameCube adapters.
* - "1": HIDAPI will use libusb for GameCube adapters if available. (default)
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.4.0.
*/
pub const SDL_HINT_HIDAPI_LIBUSB_GAMECUBE: Hint = Hint { name: "SDL_HINT_HIDAPI_LIBUSB_GAMECUBE", value: "SDL_HIDAPI_LIBUSB_GAMECUBE", doc: "/**\n* A variable to control whether HIDAPI uses libusb for GameCube adapters.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": HIDAPI will not use libusb for GameCube adapters.\n* - \"1\": HIDAPI will use libusb for GameCube adapters if available. (default)\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.4.0.\n*/\n" };
/**
* A variable to control whether HIDAPI uses libusb only for whitelisted
* devices.
*
* By default libusb will only be used for a few devices that require direct
* USB access.
*
* The variable can be set to the following values:
*
* - "0": HIDAPI will use libusb for all device access.
* - "1": HIDAPI will use libusb only for whitelisted devices. (default)
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_HIDAPI_LIBUSB_WHITELIST: Hint = Hint { name: "SDL_HINT_HIDAPI_LIBUSB_WHITELIST", value: "SDL_HIDAPI_LIBUSB_WHITELIST", doc: "/**\n* A variable to control whether HIDAPI uses libusb only for whitelisted\n* devices.\n*\n* By default libusb will only be used for a few devices that require direct\n* USB access.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": HIDAPI will use libusb for all device access.\n* - \"1\": HIDAPI will use libusb only for whitelisted devices. (default)\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable to control whether HIDAPI uses udev for device detection.
*
* The variable can be set to the following values:
*
* - "0": HIDAPI will poll for device changes.
* - "1": HIDAPI will use udev for device detection. (default)
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_HIDAPI_UDEV: Hint = Hint { name: "SDL_HINT_HIDAPI_UDEV", value: "SDL_HIDAPI_UDEV", doc: "/**\n* A variable to control whether HIDAPI uses udev for device detection.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": HIDAPI will poll for device changes.\n* - \"1\": HIDAPI will use udev for device detection. (default)\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable that specifies a GPU backend to use.
*
* By default, SDL will try all available GPU backends in a reasonable order
* until it finds one that can work, but this hint allows the app or user to
* force a specific target, such as "direct3d12" if, say, your hardware
* supports Vulkan but you want to try using D3D12 instead.
*
* This hint should be set before any GPU functions are called.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_GPU_DRIVER: Hint = Hint { name: "SDL_HINT_GPU_DRIVER", value: "SDL_GPU_DRIVER", doc: "/**\n* A variable that specifies a GPU backend to use.\n*\n* By default, SDL will try all available GPU backends in a reasonable order\n* until it finds one that can work, but this hint allows the app or user to\n* force a specific target, such as \"direct3d12\" if, say, your hardware\n* supports Vulkan but you want to try using D3D12 instead.\n*\n* This hint should be set before any GPU functions are called.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable that specifies the library name to use when loading the OpenXR
* loader.
*
* By default, SDL will try the system default name, but on some platforms
* like Windows, debug builds of the OpenXR loader have a different name, and
* are not always directly compatible with release applications. Setting this
* hint allows you to compensate for this difference in your app when
* applicable.
*
* This hint should be set before the OpenXR loader is loaded. For example,
* creating an OpenXR GPU device will load the OpenXR loader.
*/
pub const SDL_HINT_OPENXR_LIBRARY: Hint = Hint { name: "SDL_HINT_OPENXR_LIBRARY", value: "SDL_OPENXR_LIBRARY", doc: "/**\n* A variable that specifies the library name to use when loading the OpenXR\n* loader.\n*\n* By default, SDL will try the system default name, but on some platforms\n* like Windows, debug builds of the OpenXR loader have a different name, and\n* are not always directly compatible with release applications. Setting this\n* hint allows you to compensate for this difference in your app when\n* applicable.\n*\n* This hint should be set before the OpenXR loader is loaded. For example,\n* creating an OpenXR GPU device will load the OpenXR loader.\n*/\n" };
/**
* A variable to control whether SDL_hid_enumerate() enumerates all HID
* devices or only controllers.
*
* The variable can be set to the following values:
*
* - "0": SDL_hid_enumerate() will enumerate all HID devices.
* - "1": SDL_hid_enumerate() will only enumerate controllers. (default)
*
* By default SDL will only enumerate controllers, to reduce risk of hanging
* or crashing on devices with bad drivers and avoiding macOS keyboard capture
* permission prompts.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_HIDAPI_ENUMERATE_ONLY_CONTROLLERS: Hint = Hint { name: "SDL_HINT_HIDAPI_ENUMERATE_ONLY_CONTROLLERS", value: "SDL_HIDAPI_ENUMERATE_ONLY_CONTROLLERS", doc: "/**\n* A variable to control whether SDL_hid_enumerate() enumerates all HID\n* devices or only controllers.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": SDL_hid_enumerate() will enumerate all HID devices.\n* - \"1\": SDL_hid_enumerate() will only enumerate controllers. (default)\n*\n* By default SDL will only enumerate controllers, to reduce risk of hanging\n* or crashing on devices with bad drivers and avoiding macOS keyboard capture\n* permission prompts.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable containing a list of devices to ignore in SDL_hid_enumerate().
*
* The format of the string is a comma separated list of USB VID/PID pairs in
* hexadecimal form, e.g.
*
* `0xAAAA/0xBBBB,0xCCCC/0xDDDD`
*
* For example, to ignore the Shanwan DS3 controller and any Valve controller,
* you might use the string "0x2563/0x0523,0x28de/0x0000"
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_HIDAPI_IGNORE_DEVICES: Hint = Hint { name: "SDL_HINT_HIDAPI_IGNORE_DEVICES", value: "SDL_HIDAPI_IGNORE_DEVICES", doc: "/**\n* A variable containing a list of devices to ignore in SDL_hid_enumerate().\n*\n* The format of the string is a comma separated list of USB VID/PID pairs in\n* hexadecimal form, e.g.\n*\n* `0xAAAA/0xBBBB,0xCCCC/0xDDDD`\n*\n* For example, to ignore the Shanwan DS3 controller and any Valve controller,\n* you might use the string \"0x2563/0x0523,0x28de/0x0000\"\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable describing what IME UI elements the application can display.
*
* By default IME UI is handled using native components by the OS where
* possible, however this can interfere with or not be visible when exclusive
* fullscreen mode is used.
*
* The variable can be set to a comma separated list containing the following
* items:
*
* - "none" or "0": The application can't render any IME elements, and native
*   UI should be used. (default)
* - "composition": The application handles SDL_EVENT_TEXT_EDITING events and
*   can render the composition text.
* - "candidates": The application handles SDL_EVENT_TEXT_EDITING_CANDIDATES
*   and can render the candidate list.
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_IME_IMPLEMENTED_UI: Hint = Hint { name: "SDL_HINT_IME_IMPLEMENTED_UI", value: "SDL_IME_IMPLEMENTED_UI", doc: "/**\n* A variable describing what IME UI elements the application can display.\n*\n* By default IME UI is handled using native components by the OS where\n* possible, however this can interfere with or not be visible when exclusive\n* fullscreen mode is used.\n*\n* The variable can be set to a comma separated list containing the following\n* items:\n*\n* - \"none\" or \"0\": The application can't render any IME elements, and native\n*   UI should be used. (default)\n* - \"composition\": The application handles SDL_EVENT_TEXT_EDITING events and\n*   can render the composition text.\n* - \"candidates\": The application handles SDL_EVENT_TEXT_EDITING_CANDIDATES\n*   and can render the candidate list.\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the home indicator bar on iPhone X and later
* should be hidden.
*
* The variable can be set to the following values:
*
* - "0": The indicator bar is not hidden. (default for windowed applications)
* - "1": The indicator bar is hidden and is shown when the screen is touched
*   (useful for movie playback applications).
* - "2": The indicator bar is dim and the first swipe makes it visible and
*   the second swipe performs the "home" action. (default for fullscreen
*   applications)
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_IOS_HIDE_HOME_INDICATOR: Hint = Hint { name: "SDL_HINT_IOS_HIDE_HOME_INDICATOR", value: "SDL_IOS_HIDE_HOME_INDICATOR", doc: "/**\n* A variable controlling whether the home indicator bar on iPhone X and later\n* should be hidden.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": The indicator bar is not hidden. (default for windowed applications)\n* - \"1\": The indicator bar is hidden and is shown when the screen is touched\n*   (useful for movie playback applications).\n* - \"2\": The indicator bar is dim and the first swipe makes it visible and\n*   the second swipe performs the \"home\" action. (default for fullscreen\n*   applications)\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable that lets you enable joystick (and gamecontroller) events even
* when your app is in the background.
*
* The variable can be set to the following values:
*
* - "0": Disable joystick & gamecontroller input events when the application
*   is in the background. (default)
* - "1": Enable joystick & gamecontroller input events when the application
*   is in the background.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_ALLOW_BACKGROUND_EVENTS: Hint = Hint { name: "SDL_HINT_JOYSTICK_ALLOW_BACKGROUND_EVENTS", value: "SDL_JOYSTICK_ALLOW_BACKGROUND_EVENTS", doc: "/**\n* A variable that lets you enable joystick (and gamecontroller) events even\n* when your app is in the background.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Disable joystick & gamecontroller input events when the application\n*   is in the background. (default)\n* - \"1\": Enable joystick & gamecontroller input events when the application\n*   is in the background.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable containing a list of arcade stick style controllers.
*
* The format of the string is a comma separated list of USB VID/PID pairs in
* hexadecimal form, e.g.
*
* `0xAAAA/0xBBBB,0xCCCC/0xDDDD`
*
* The variable can also take the form of "@file", in which case the named
* file will be loaded and interpreted as the value of the variable.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_ARCADESTICK_DEVICES: Hint = Hint { name: "SDL_HINT_JOYSTICK_ARCADESTICK_DEVICES", value: "SDL_JOYSTICK_ARCADESTICK_DEVICES", doc: "/**\n* A variable containing a list of arcade stick style controllers.\n*\n* The format of the string is a comma separated list of USB VID/PID pairs in\n* hexadecimal form, e.g.\n*\n* `0xAAAA/0xBBBB,0xCCCC/0xDDDD`\n*\n* The variable can also take the form of \"@file\", in which case the named\n* file will be loaded and interpreted as the value of the variable.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable containing a list of devices that are not arcade stick style
* controllers.
*
* This will override SDL_HINT_JOYSTICK_ARCADESTICK_DEVICES and the built in
* device list.
*
* The format of the string is a comma separated list of USB VID/PID pairs in
* hexadecimal form, e.g.
*
* `0xAAAA/0xBBBB,0xCCCC/0xDDDD`
*
* The variable can also take the form of "@file", in which case the named
* file will be loaded and interpreted as the value of the variable.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_ARCADESTICK_DEVICES_EXCLUDED: Hint = Hint { name: "SDL_HINT_JOYSTICK_ARCADESTICK_DEVICES_EXCLUDED", value: "SDL_JOYSTICK_ARCADESTICK_DEVICES_EXCLUDED", doc: "/**\n* A variable containing a list of devices that are not arcade stick style\n* controllers.\n*\n* This will override SDL_HINT_JOYSTICK_ARCADESTICK_DEVICES and the built in\n* device list.\n*\n* The format of the string is a comma separated list of USB VID/PID pairs in\n* hexadecimal form, e.g.\n*\n* `0xAAAA/0xBBBB,0xCCCC/0xDDDD`\n*\n* The variable can also take the form of \"@file\", in which case the named\n* file will be loaded and interpreted as the value of the variable.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable containing a list of devices that should not be considered
* joysticks.
*
* The format of the string is a comma separated list of USB VID/PID pairs in
* hexadecimal form, e.g.
*
* `0xAAAA/0xBBBB,0xCCCC/0xDDDD`
*
* The variable can also take the form of "@file", in which case the named
* file will be loaded and interpreted as the value of the variable.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_BLACKLIST_DEVICES: Hint = Hint { name: "SDL_HINT_JOYSTICK_BLACKLIST_DEVICES", value: "SDL_JOYSTICK_BLACKLIST_DEVICES", doc: "/**\n* A variable containing a list of devices that should not be considered\n* joysticks.\n*\n* The format of the string is a comma separated list of USB VID/PID pairs in\n* hexadecimal form, e.g.\n*\n* `0xAAAA/0xBBBB,0xCCCC/0xDDDD`\n*\n* The variable can also take the form of \"@file\", in which case the named\n* file will be loaded and interpreted as the value of the variable.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable containing a list of devices that should be considered
* joysticks.
*
* This will override SDL_HINT_JOYSTICK_BLACKLIST_DEVICES and the built in
* device list.
*
* The format of the string is a comma separated list of USB VID/PID pairs in
* hexadecimal form, e.g.
*
* `0xAAAA/0xBBBB,0xCCCC/0xDDDD`
*
* The variable can also take the form of "@file", in which case the named
* file will be loaded and interpreted as the value of the variable.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_BLACKLIST_DEVICES_EXCLUDED: Hint = Hint { name: "SDL_HINT_JOYSTICK_BLACKLIST_DEVICES_EXCLUDED", value: "SDL_JOYSTICK_BLACKLIST_DEVICES_EXCLUDED", doc: "/**\n* A variable containing a list of devices that should be considered\n* joysticks.\n*\n* This will override SDL_HINT_JOYSTICK_BLACKLIST_DEVICES and the built in\n* device list.\n*\n* The format of the string is a comma separated list of USB VID/PID pairs in\n* hexadecimal form, e.g.\n*\n* `0xAAAA/0xBBBB,0xCCCC/0xDDDD`\n*\n* The variable can also take the form of \"@file\", in which case the named\n* file will be loaded and interpreted as the value of the variable.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable containing a comma separated list of devices to open as
* joysticks.
*
* This variable is currently only used by the Linux joystick driver.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_DEVICE: Hint = Hint { name: "SDL_HINT_JOYSTICK_DEVICE", value: "SDL_JOYSTICK_DEVICE", doc: "/**\n* A variable containing a comma separated list of devices to open as\n* joysticks.\n*\n* This variable is currently only used by the Linux joystick driver.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable containing a list of drum style controllers.
*
* The format of the string is a comma separated list of USB VID/PID pairs in
* hexadecimal form, e.g.
*
* `0xAAAA/0xBBBB,0xCCCC/0xDDDD`
*
* The variable can also take the form of "@file", in which case the named
* file will be loaded and interpreted as the value of the variable.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.4.4.
*/
pub const SDL_HINT_JOYSTICK_DRUM_DEVICES: Hint = Hint { name: "SDL_HINT_JOYSTICK_DRUM_DEVICES", value: "SDL_JOYSTICK_DRUM_DEVICES", doc: "/**\n* A variable containing a list of drum style controllers.\n*\n* The format of the string is a comma separated list of USB VID/PID pairs in\n* hexadecimal form, e.g.\n*\n* `0xAAAA/0xBBBB,0xCCCC/0xDDDD`\n*\n* The variable can also take the form of \"@file\", in which case the named\n* file will be loaded and interpreted as the value of the variable.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.4.4.\n*/\n" };
/**
* A variable controlling whether enhanced reports should be used for
* controllers when using the HIDAPI driver.
*
* Enhanced reports allow rumble and effects on Bluetooth PlayStation
* controllers and gyro on Nintendo Switch controllers, but break Windows
* DirectInput for other applications that don't use SDL.
*
* Once enhanced reports are enabled, they can't be disabled on PlayStation
* controllers without power cycling the controller.
*
* The variable can be set to the following values:
*
* - "0": enhanced reports are not enabled.
* - "1": enhanced reports are enabled. (default)
* - "auto": enhanced features are advertised to the application, but SDL
*   doesn't change the controller report mode unless the application uses
*   them.
*
* This hint can be enabled anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_ENHANCED_REPORTS: Hint = Hint { name: "SDL_HINT_JOYSTICK_ENHANCED_REPORTS", value: "SDL_JOYSTICK_ENHANCED_REPORTS", doc: "/**\n* A variable controlling whether enhanced reports should be used for\n* controllers when using the HIDAPI driver.\n*\n* Enhanced reports allow rumble and effects on Bluetooth PlayStation\n* controllers and gyro on Nintendo Switch controllers, but break Windows\n* DirectInput for other applications that don't use SDL.\n*\n* Once enhanced reports are enabled, they can't be disabled on PlayStation\n* controllers without power cycling the controller.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": enhanced reports are not enabled.\n* - \"1\": enhanced reports are enabled. (default)\n* - \"auto\": enhanced features are advertised to the application, but SDL\n*   doesn't change the controller report mode unless the application uses\n*   them.\n*\n* This hint can be enabled anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable containing a list of flightstick style controllers.
*
* The format of the string is a comma separated list of USB VID/PID pairs in
* hexadecimal form, e.g.
*
* `0xAAAA/0xBBBB,0xCCCC/0xDDDD`
*
* The variable can also take the form of @file, in which case the named file
* will be loaded and interpreted as the value of the variable.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_FLIGHTSTICK_DEVICES: Hint = Hint { name: "SDL_HINT_JOYSTICK_FLIGHTSTICK_DEVICES", value: "SDL_JOYSTICK_FLIGHTSTICK_DEVICES", doc: "/**\n* A variable containing a list of flightstick style controllers.\n*\n* The format of the string is a comma separated list of USB VID/PID pairs in\n* hexadecimal form, e.g.\n*\n* `0xAAAA/0xBBBB,0xCCCC/0xDDDD`\n*\n* The variable can also take the form of @file, in which case the named file\n* will be loaded and interpreted as the value of the variable.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable containing a list of devices that are not flightstick style
* controllers.
*
* This will override SDL_HINT_JOYSTICK_FLIGHTSTICK_DEVICES and the built in
* device list.
*
* The format of the string is a comma separated list of USB VID/PID pairs in
* hexadecimal form, e.g.
*
* `0xAAAA/0xBBBB,0xCCCC/0xDDDD`
*
* The variable can also take the form of "@file", in which case the named
* file will be loaded and interpreted as the value of the variable.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_FLIGHTSTICK_DEVICES_EXCLUDED: Hint = Hint { name: "SDL_HINT_JOYSTICK_FLIGHTSTICK_DEVICES_EXCLUDED", value: "SDL_JOYSTICK_FLIGHTSTICK_DEVICES_EXCLUDED", doc: "/**\n* A variable containing a list of devices that are not flightstick style\n* controllers.\n*\n* This will override SDL_HINT_JOYSTICK_FLIGHTSTICK_DEVICES and the built in\n* device list.\n*\n* The format of the string is a comma separated list of USB VID/PID pairs in\n* hexadecimal form, e.g.\n*\n* `0xAAAA/0xBBBB,0xCCCC/0xDDDD`\n*\n* The variable can also take the form of \"@file\", in which case the named\n* file will be loaded and interpreted as the value of the variable.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether GameInput should be used for controller
* handling on Windows.
*
* The variable can be set to the following values:
*
* - "0": GameInput is not used.
* - "1": GameInput is used.
*
* The default is "1" on GDK platforms, and "0" otherwise.
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_GAMEINPUT: Hint = Hint { name: "SDL_HINT_JOYSTICK_GAMEINPUT", value: "SDL_JOYSTICK_GAMEINPUT", doc: "/**\n* A variable controlling whether GameInput should be used for controller\n* handling on Windows.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": GameInput is not used.\n* - \"1\": GameInput is used.\n*\n* The default is \"1\" on GDK platforms, and \"0\" otherwise.\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether GameInput should be used for handling GIP
* devices that require raw report processing, but aren't supported by HIDRAW,
* such as Xbox One Guitars.
*
* Note that this is only supported with GameInput 3 or newer.
*
* The variable can be set to the following values:
*
* - "0": GameInput is not used to handle raw GIP devices.
* - "1": GameInput is used.
*
* The default is "1" when using GameInput 3 or newer, and is "0" otherwise.
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.4.4.
*/
pub const SDL_HINT_JOYSTICK_GAMEINPUT_RAW: Hint = Hint { name: "SDL_HINT_JOYSTICK_GAMEINPUT_RAW", value: "SDL_JOYSTICK_GAMEINPUT_RAW", doc: "/**\n* A variable controlling whether GameInput should be used for handling GIP\n* devices that require raw report processing, but aren't supported by HIDRAW,\n* such as Xbox One Guitars.\n*\n* Note that this is only supported with GameInput 3 or newer.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": GameInput is not used to handle raw GIP devices.\n* - \"1\": GameInput is used.\n*\n* The default is \"1\" when using GameInput 3 or newer, and is \"0\" otherwise.\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.4.4.\n*/\n" };
/**
* A variable containing a list of devices known to have a GameCube form
* factor.
*
* The format of the string is a comma separated list of USB VID/PID pairs in
* hexadecimal form, e.g.
*
* `0xAAAA/0xBBBB,0xCCCC/0xDDDD`
*
* The variable can also take the form of "@file", in which case the named
* file will be loaded and interpreted as the value of the variable.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_GAMECUBE_DEVICES: Hint = Hint { name: "SDL_HINT_JOYSTICK_GAMECUBE_DEVICES", value: "SDL_JOYSTICK_GAMECUBE_DEVICES", doc: "/**\n* A variable containing a list of devices known to have a GameCube form\n* factor.\n*\n* The format of the string is a comma separated list of USB VID/PID pairs in\n* hexadecimal form, e.g.\n*\n* `0xAAAA/0xBBBB,0xCCCC/0xDDDD`\n*\n* The variable can also take the form of \"@file\", in which case the named\n* file will be loaded and interpreted as the value of the variable.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable containing a list of devices known not to have a GameCube form
* factor.
*
* This will override SDL_HINT_JOYSTICK_GAMECUBE_DEVICES and the built in
* device list.
*
* The format of the string is a comma separated list of USB VID/PID pairs in
* hexadecimal form, e.g.
*
* `0xAAAA/0xBBBB,0xCCCC/0xDDDD`
*
* The variable can also take the form of "@file", in which case the named
* file will be loaded and interpreted as the value of the variable.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_GAMECUBE_DEVICES_EXCLUDED: Hint = Hint { name: "SDL_HINT_JOYSTICK_GAMECUBE_DEVICES_EXCLUDED", value: "SDL_JOYSTICK_GAMECUBE_DEVICES_EXCLUDED", doc: "/**\n* A variable containing a list of devices known not to have a GameCube form\n* factor.\n*\n* This will override SDL_HINT_JOYSTICK_GAMECUBE_DEVICES and the built in\n* device list.\n*\n* The format of the string is a comma separated list of USB VID/PID pairs in\n* hexadecimal form, e.g.\n*\n* `0xAAAA/0xBBBB,0xCCCC/0xDDDD`\n*\n* The variable can also take the form of \"@file\", in which case the named\n* file will be loaded and interpreted as the value of the variable.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable containing a list of guitar style controllers.
*
* The format of the string is a comma separated list of USB VID/PID pairs in
* hexadecimal form, e.g.
*
* `0xAAAA/0xBBBB,0xCCCC/0xDDDD`
*
* The variable can also take the form of "@file", in which case the named
* file will be loaded and interpreted as the value of the variable.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.4.4.
*/
pub const SDL_HINT_JOYSTICK_GUITAR_DEVICES: Hint = Hint { name: "SDL_HINT_JOYSTICK_GUITAR_DEVICES", value: "SDL_JOYSTICK_GUITAR_DEVICES", doc: "/**\n* A variable containing a list of guitar style controllers.\n*\n* The format of the string is a comma separated list of USB VID/PID pairs in\n* hexadecimal form, e.g.\n*\n* `0xAAAA/0xBBBB,0xCCCC/0xDDDD`\n*\n* The variable can also take the form of \"@file\", in which case the named\n* file will be loaded and interpreted as the value of the variable.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.4.4.\n*/\n" };
/**
* A variable controlling whether the HIDAPI joystick drivers should be used.
*
* The variable can be set to the following values:
*
* - "0": HIDAPI drivers are not used.
* - "1": HIDAPI drivers are used. (default)
*
* This variable is the default for all drivers, but can be overridden by the
* hints for specific drivers below.
*
* This hint should be set before initializing joysticks and gamepads.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_HIDAPI: Hint = Hint { name: "SDL_HINT_JOYSTICK_HIDAPI", value: "SDL_JOYSTICK_HIDAPI", doc: "/**\n* A variable controlling whether the HIDAPI joystick drivers should be used.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": HIDAPI drivers are not used.\n* - \"1\": HIDAPI drivers are used. (default)\n*\n* This variable is the default for all drivers, but can be overridden by the\n* hints for specific drivers below.\n*\n* This hint should be set before initializing joysticks and gamepads.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether Nintendo Switch Joy-Con controllers will be
* combined into a single Pro-like controller when using the HIDAPI driver.
*
* The variable can be set to the following values:
*
* - "0": Left and right Joy-Con controllers will not be combined and each
*   will be a mini-gamepad.
* - "1": Left and right Joy-Con controllers will be combined into a single
*   controller. (default)
*
* This hint should be set before initializing joysticks and gamepads.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_HIDAPI_COMBINE_JOY_CONS: Hint = Hint { name: "SDL_HINT_JOYSTICK_HIDAPI_COMBINE_JOY_CONS", value: "SDL_JOYSTICK_HIDAPI_COMBINE_JOY_CONS", doc: "/**\n* A variable controlling whether Nintendo Switch Joy-Con controllers will be\n* combined into a single Pro-like controller when using the HIDAPI driver.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Left and right Joy-Con controllers will not be combined and each\n*   will be a mini-gamepad.\n* - \"1\": Left and right Joy-Con controllers will be combined into a single\n*   controller. (default)\n*\n* This hint should be set before initializing joysticks and gamepads.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the HIDAPI driver for Nintendo GameCube
* controllers should be used.
*
* The variable can be set to the following values:
*
* - "0": HIDAPI driver is not used.
* - "1": HIDAPI driver is used.
*
* The default is the value of SDL_HINT_JOYSTICK_HIDAPI
*
* This hint should be set before initializing joysticks and gamepads.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_HIDAPI_GAMECUBE: Hint = Hint { name: "SDL_HINT_JOYSTICK_HIDAPI_GAMECUBE", value: "SDL_JOYSTICK_HIDAPI_GAMECUBE", doc: "/**\n* A variable controlling whether the HIDAPI driver for Nintendo GameCube\n* controllers should be used.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": HIDAPI driver is not used.\n* - \"1\": HIDAPI driver is used.\n*\n* The default is the value of SDL_HINT_JOYSTICK_HIDAPI\n*\n* This hint should be set before initializing joysticks and gamepads.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether rumble is used to implement the GameCube
* controller's 3 rumble modes, Stop(0), Rumble(1), and StopHard(2).
*
* This is useful for applications that need full compatibility for things
* like ADSR envelopes. - Stop is implemented by setting low_frequency_rumble
* to 0 and high_frequency_rumble >0 - Rumble is both at any arbitrary value -
* StopHard is implemented by setting both low_frequency_rumble and
* high_frequency_rumble to 0
*
* The variable can be set to the following values:
*
* - "0": Normal rumble behavior is behavior is used. (default)
* - "1": Proper GameCube controller rumble behavior is used.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_HIDAPI_GAMECUBE_RUMBLE_BRAKE: Hint = Hint { name: "SDL_HINT_JOYSTICK_HIDAPI_GAMECUBE_RUMBLE_BRAKE", value: "SDL_JOYSTICK_HIDAPI_GAMECUBE_RUMBLE_BRAKE", doc: "/**\n* A variable controlling whether rumble is used to implement the GameCube\n* controller's 3 rumble modes, Stop(0), Rumble(1), and StopHard(2).\n*\n* This is useful for applications that need full compatibility for things\n* like ADSR envelopes. - Stop is implemented by setting low_frequency_rumble\n* to 0 and high_frequency_rumble >0 - Rumble is both at any arbitrary value -\n* StopHard is implemented by setting both low_frequency_rumble and\n* high_frequency_rumble to 0\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Normal rumble behavior is behavior is used. (default)\n* - \"1\": Proper GameCube controller rumble behavior is used.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the HIDAPI driver for Nintendo Switch
* Joy-Cons should be used.
*
* The variable can be set to the following values:
*
* - "0": HIDAPI driver is not used.
* - "1": HIDAPI driver is used.
*
* The default is the value of SDL_HINT_JOYSTICK_HIDAPI.
*
* This hint should be set before initializing joysticks and gamepads.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_HIDAPI_JOY_CONS: Hint = Hint { name: "SDL_HINT_JOYSTICK_HIDAPI_JOY_CONS", value: "SDL_JOYSTICK_HIDAPI_JOY_CONS", doc: "/**\n* A variable controlling whether the HIDAPI driver for Nintendo Switch\n* Joy-Cons should be used.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": HIDAPI driver is not used.\n* - \"1\": HIDAPI driver is used.\n*\n* The default is the value of SDL_HINT_JOYSTICK_HIDAPI.\n*\n* This hint should be set before initializing joysticks and gamepads.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the Home button LED should be turned on when
* a Nintendo Switch Joy-Con controller is opened.
*
* The variable can be set to the following values:
*
* - "0": home button LED is turned off
* - "1": home button LED is turned on
*
* By default the Home button LED state is not changed. This hint can also be
* set to a floating point value between 0.0 and 1.0 which controls the
* brightness of the Home button LED.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_HIDAPI_JOYCON_HOME_LED: Hint = Hint { name: "SDL_HINT_JOYSTICK_HIDAPI_JOYCON_HOME_LED", value: "SDL_JOYSTICK_HIDAPI_JOYCON_HOME_LED", doc: "/**\n* A variable controlling whether the Home button LED should be turned on when\n* a Nintendo Switch Joy-Con controller is opened.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": home button LED is turned off\n* - \"1\": home button LED is turned on\n*\n* By default the Home button LED state is not changed. This hint can also be\n* set to a floating point value between 0.0 and 1.0 which controls the\n* brightness of the Home button LED.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the HIDAPI driver for Amazon Luna
* controllers connected via Bluetooth should be used.
*
* The variable can be set to the following values:
*
* - "0": HIDAPI driver is not used.
* - "1": HIDAPI driver is used.
*
* The default is the value of SDL_HINT_JOYSTICK_HIDAPI.
*
* This hint should be set before initializing joysticks and gamepads.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_HIDAPI_LUNA: Hint = Hint { name: "SDL_HINT_JOYSTICK_HIDAPI_LUNA", value: "SDL_JOYSTICK_HIDAPI_LUNA", doc: "/**\n* A variable controlling whether the HIDAPI driver for Amazon Luna\n* controllers connected via Bluetooth should be used.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": HIDAPI driver is not used.\n* - \"1\": HIDAPI driver is used.\n*\n* The default is the value of SDL_HINT_JOYSTICK_HIDAPI.\n*\n* This hint should be set before initializing joysticks and gamepads.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the HIDAPI driver for Nintendo Online
* classic controllers should be used.
*
* The variable can be set to the following values:
*
* - "0": HIDAPI driver is not used.
* - "1": HIDAPI driver is used.
*
* The default is the value of SDL_HINT_JOYSTICK_HIDAPI.
*
* This hint should be set before initializing joysticks and gamepads.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_HIDAPI_NINTENDO_CLASSIC: Hint = Hint { name: "SDL_HINT_JOYSTICK_HIDAPI_NINTENDO_CLASSIC", value: "SDL_JOYSTICK_HIDAPI_NINTENDO_CLASSIC", doc: "/**\n* A variable controlling whether the HIDAPI driver for Nintendo Online\n* classic controllers should be used.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": HIDAPI driver is not used.\n* - \"1\": HIDAPI driver is used.\n*\n* The default is the value of SDL_HINT_JOYSTICK_HIDAPI.\n*\n* This hint should be set before initializing joysticks and gamepads.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the HIDAPI driver for PS3 controllers should
* be used.
*
* The variable can be set to the following values:
*
* - "0": HIDAPI driver is not used.
* - "1": HIDAPI driver is used.
*
* The default is the value of SDL_HINT_JOYSTICK_HIDAPI on macOS, and "0" on
* other platforms.
*
* For official Sony driver (sixaxis.sys) use
* SDL_HINT_JOYSTICK_HIDAPI_PS3_SIXAXIS_DRIVER. See
* https://github.com/ViGEm/DsHidMini for an alternative driver on Windows.
*
* This hint should be set before initializing joysticks and gamepads.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_HIDAPI_PS3: Hint = Hint { name: "SDL_HINT_JOYSTICK_HIDAPI_PS3", value: "SDL_JOYSTICK_HIDAPI_PS3", doc: "/**\n* A variable controlling whether the HIDAPI driver for PS3 controllers should\n* be used.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": HIDAPI driver is not used.\n* - \"1\": HIDAPI driver is used.\n*\n* The default is the value of SDL_HINT_JOYSTICK_HIDAPI on macOS, and \"0\" on\n* other platforms.\n*\n* For official Sony driver (sixaxis.sys) use\n* SDL_HINT_JOYSTICK_HIDAPI_PS3_SIXAXIS_DRIVER. See\n* https://github.com/ViGEm/DsHidMini for an alternative driver on Windows.\n*\n* This hint should be set before initializing joysticks and gamepads.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the Sony driver (sixaxis.sys) for PS3
* controllers (Sixaxis/DualShock 3) should be used.
*
* The variable can be set to the following values:
*
* - "0": Sony driver (sixaxis.sys) is not used.
* - "1": Sony driver (sixaxis.sys) is used.
*
* The default value is 0.
*
* This hint should be set before initializing joysticks and gamepads.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_HIDAPI_PS3_SIXAXIS_DRIVER: Hint = Hint { name: "SDL_HINT_JOYSTICK_HIDAPI_PS3_SIXAXIS_DRIVER", value: "SDL_JOYSTICK_HIDAPI_PS3_SIXAXIS_DRIVER", doc: "/**\n* A variable controlling whether the Sony driver (sixaxis.sys) for PS3\n* controllers (Sixaxis/DualShock 3) should be used.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Sony driver (sixaxis.sys) is not used.\n* - \"1\": Sony driver (sixaxis.sys) is used.\n*\n* The default value is 0.\n*\n* This hint should be set before initializing joysticks and gamepads.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the HIDAPI driver for PS4 controllers should
* be used.
*
* The variable can be set to the following values:
*
* - "0": HIDAPI driver is not used.
* - "1": HIDAPI driver is used.
*
* The default is the value of SDL_HINT_JOYSTICK_HIDAPI.
*
* This hint should be set before initializing joysticks and gamepads.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_HIDAPI_PS4: Hint = Hint { name: "SDL_HINT_JOYSTICK_HIDAPI_PS4", value: "SDL_JOYSTICK_HIDAPI_PS4", doc: "/**\n* A variable controlling whether the HIDAPI driver for PS4 controllers should\n* be used.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": HIDAPI driver is not used.\n* - \"1\": HIDAPI driver is used.\n*\n* The default is the value of SDL_HINT_JOYSTICK_HIDAPI.\n*\n* This hint should be set before initializing joysticks and gamepads.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling the update rate of the PS4 controller over Bluetooth
* when using the HIDAPI driver.
*
* This defaults to 4 ms, to match the behavior over USB, and to be more
* friendly to other Bluetooth devices and older Bluetooth hardware on the
* computer. It can be set to "1" (1000Hz), "2" (500Hz) and "4" (250Hz)
*
* This hint can be set anytime, but only takes effect when extended input
* reports are enabled.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_HIDAPI_PS4_REPORT_INTERVAL: Hint = Hint { name: "SDL_HINT_JOYSTICK_HIDAPI_PS4_REPORT_INTERVAL", value: "SDL_JOYSTICK_HIDAPI_PS4_REPORT_INTERVAL", doc: "/**\n* A variable controlling the update rate of the PS4 controller over Bluetooth\n* when using the HIDAPI driver.\n*\n* This defaults to 4 ms, to match the behavior over USB, and to be more\n* friendly to other Bluetooth devices and older Bluetooth hardware on the\n* computer. It can be set to \"1\" (1000Hz), \"2\" (500Hz) and \"4\" (250Hz)\n*\n* This hint can be set anytime, but only takes effect when extended input\n* reports are enabled.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the HIDAPI driver for PS5 controllers should
* be used.
*
* The variable can be set to the following values:
*
* - "0": HIDAPI driver is not used.
* - "1": HIDAPI driver is used.
*
* The default is the value of SDL_HINT_JOYSTICK_HIDAPI.
*
* This hint should be set before initializing joysticks and gamepads.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_HIDAPI_PS5: Hint = Hint { name: "SDL_HINT_JOYSTICK_HIDAPI_PS5", value: "SDL_JOYSTICK_HIDAPI_PS5", doc: "/**\n* A variable controlling whether the HIDAPI driver for PS5 controllers should\n* be used.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": HIDAPI driver is not used.\n* - \"1\": HIDAPI driver is used.\n*\n* The default is the value of SDL_HINT_JOYSTICK_HIDAPI.\n*\n* This hint should be set before initializing joysticks and gamepads.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the player LEDs should be lit to indicate
* which player is associated with a PS5 controller.
*
* The variable can be set to the following values:
*
* - "0": player LEDs are not enabled.
* - "1": player LEDs are enabled. (default)
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_HIDAPI_PS5_PLAYER_LED: Hint = Hint { name: "SDL_HINT_JOYSTICK_HIDAPI_PS5_PLAYER_LED", value: "SDL_JOYSTICK_HIDAPI_PS5_PLAYER_LED", doc: "/**\n* A variable controlling whether the player LEDs should be lit to indicate\n* which player is associated with a PS5 controller.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": player LEDs are not enabled.\n* - \"1\": player LEDs are enabled. (default)\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the HIDAPI driver for NVIDIA SHIELD
* controllers should be used.
*
* The variable can be set to the following values:
*
* - "0": HIDAPI driver is not used.
* - "1": HIDAPI driver is used.
*
* The default is the value of SDL_HINT_JOYSTICK_HIDAPI.
*
* This hint should be set before initializing joysticks and gamepads.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_HIDAPI_SHIELD: Hint = Hint { name: "SDL_HINT_JOYSTICK_HIDAPI_SHIELD", value: "SDL_JOYSTICK_HIDAPI_SHIELD", doc: "/**\n* A variable controlling whether the HIDAPI driver for NVIDIA SHIELD\n* controllers should be used.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": HIDAPI driver is not used.\n* - \"1\": HIDAPI driver is used.\n*\n* The default is the value of SDL_HINT_JOYSTICK_HIDAPI.\n*\n* This hint should be set before initializing joysticks and gamepads.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the HIDAPI driver for Google Stadia
* controllers should be used.
*
* The variable can be set to the following values:
*
* - "0": HIDAPI driver is not used.
* - "1": HIDAPI driver is used.
*
* The default is the value of SDL_HINT_JOYSTICK_HIDAPI.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_HIDAPI_STADIA: Hint = Hint { name: "SDL_HINT_JOYSTICK_HIDAPI_STADIA", value: "SDL_JOYSTICK_HIDAPI_STADIA", doc: "/**\n* A variable controlling whether the HIDAPI driver for Google Stadia\n* controllers should be used.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": HIDAPI driver is not used.\n* - \"1\": HIDAPI driver is used.\n*\n* The default is the value of SDL_HINT_JOYSTICK_HIDAPI.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the HIDAPI driver for Bluetooth Steam
* Controllers should be used.
*
* The variable can be set to the following values:
*
* - "0": HIDAPI driver is not used. (default)
* - "1": HIDAPI driver is used for Steam Controllers, which requires
*   Bluetooth access and may prompt the user for permission on iOS and
*   Android.
*
* This hint should be set before initializing joysticks and gamepads.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_HIDAPI_STEAM: Hint = Hint { name: "SDL_HINT_JOYSTICK_HIDAPI_STEAM", value: "SDL_JOYSTICK_HIDAPI_STEAM", doc: "/**\n* A variable controlling whether the HIDAPI driver for Bluetooth Steam\n* Controllers should be used.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": HIDAPI driver is not used. (default)\n* - \"1\": HIDAPI driver is used for Steam Controllers, which requires\n*   Bluetooth access and may prompt the user for permission on iOS and\n*   Android.\n*\n* This hint should be set before initializing joysticks and gamepads.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the Steam button LED should be turned on
* when a Steam controller is opened.
*
* The variable can be set to the following values:
*
* - "0": Steam button LED is turned off.
* - "1": Steam button LED is turned on.
*
* By default the Steam button LED state is not changed. This hint can also be
* set to a floating point value between 0.0 and 1.0 which controls the
* brightness of the Steam button LED.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_HIDAPI_STEAM_HOME_LED: Hint = Hint { name: "SDL_HINT_JOYSTICK_HIDAPI_STEAM_HOME_LED", value: "SDL_JOYSTICK_HIDAPI_STEAM_HOME_LED", doc: "/**\n* A variable controlling whether the Steam button LED should be turned on\n* when a Steam controller is opened.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Steam button LED is turned off.\n* - \"1\": Steam button LED is turned on.\n*\n* By default the Steam button LED state is not changed. This hint can also be\n* set to a floating point value between 0.0 and 1.0 which controls the\n* brightness of the Steam button LED.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the HIDAPI driver for the Steam Deck builtin
* controller should be used.
*
* The variable can be set to the following values:
*
* - "0": HIDAPI driver is not used.
* - "1": HIDAPI driver is used.
*
* The default is the value of SDL_HINT_JOYSTICK_HIDAPI.
*
* This hint should be set before initializing joysticks and gamepads.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_HIDAPI_STEAMDECK: Hint = Hint { name: "SDL_HINT_JOYSTICK_HIDAPI_STEAMDECK", value: "SDL_JOYSTICK_HIDAPI_STEAMDECK", doc: "/**\n* A variable controlling whether the HIDAPI driver for the Steam Deck builtin\n* controller should be used.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": HIDAPI driver is not used.\n* - \"1\": HIDAPI driver is used.\n*\n* The default is the value of SDL_HINT_JOYSTICK_HIDAPI.\n*\n* This hint should be set before initializing joysticks and gamepads.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the HIDAPI driver for HORI licensed Steam
* controllers should be used.
*
* The variable can be set to the following values:
*
* - "0": HIDAPI driver is not used.
* - "1": HIDAPI driver is used.
*
* The default is the value of SDL_HINT_JOYSTICK_HIDAPI.
*
* This hint should be set before initializing joysticks and gamepads.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_HIDAPI_STEAM_HORI: Hint = Hint { name: "SDL_HINT_JOYSTICK_HIDAPI_STEAM_HORI", value: "SDL_JOYSTICK_HIDAPI_STEAM_HORI", doc: "/**\n* A variable controlling whether the HIDAPI driver for HORI licensed Steam\n* controllers should be used.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": HIDAPI driver is not used.\n* - \"1\": HIDAPI driver is used.\n*\n* The default is the value of SDL_HINT_JOYSTICK_HIDAPI.\n*\n* This hint should be set before initializing joysticks and gamepads.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the HIDAPI driver for some Logitech wheels
* should be used.
*
* The variable can be set to the following values:
*
* - "0": HIDAPI driver is not used.
* - "1": HIDAPI driver is used.
*
* The default is the value of SDL_HINT_JOYSTICK_HIDAPI.
*
* This hint should be set before initializing joysticks and gamepads.
*
* \since This hint is available since SDL 3.4.0.
*/
pub const SDL_HINT_JOYSTICK_HIDAPI_LG4FF: Hint = Hint { name: "SDL_HINT_JOYSTICK_HIDAPI_LG4FF", value: "SDL_JOYSTICK_HIDAPI_LG4FF", doc: "/**\n* A variable controlling whether the HIDAPI driver for some Logitech wheels\n* should be used.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": HIDAPI driver is not used.\n* - \"1\": HIDAPI driver is used.\n*\n* The default is the value of SDL_HINT_JOYSTICK_HIDAPI.\n*\n* This hint should be set before initializing joysticks and gamepads.\n*\n* \\since This hint is available since SDL 3.4.0.\n*/\n" };
/**
* A variable controlling whether the HIDAPI driver for 8BitDo controllers
* should be used.
*
* The variable can be set to the following values:
*
* - "0": HIDAPI driver is not used.
* - "1": HIDAPI driver is used.
*
* The default is the value of SDL_HINT_JOYSTICK_HIDAPI.
*
* This hint should be set before initializing joysticks and gamepads.
*
* \since This hint is available since SDL 3.4.0.
*/
pub const SDL_HINT_JOYSTICK_HIDAPI_8BITDO: Hint = Hint { name: "SDL_HINT_JOYSTICK_HIDAPI_8BITDO", value: "SDL_JOYSTICK_HIDAPI_8BITDO", doc: "/**\n* A variable controlling whether the HIDAPI driver for 8BitDo controllers\n* should be used.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": HIDAPI driver is not used.\n* - \"1\": HIDAPI driver is used.\n*\n* The default is the value of SDL_HINT_JOYSTICK_HIDAPI.\n*\n* This hint should be set before initializing joysticks and gamepads.\n*\n* \\since This hint is available since SDL 3.4.0.\n*/\n" };
/**
* A variable controlling whether the HIDAPI driver for SInput controllers
* should be used.
*
* More info - https://github.com/HandHeldLegend/SInput-HID
*
* The variable can be set to the following values:
*
* - "0": HIDAPI driver is not used.
* - "1": HIDAPI driver is used.
*
* The default is the value of SDL_HINT_JOYSTICK_HIDAPI.
*
* This hint should be set before initializing joysticks and gamepads.
*
* \since This hint is available since SDL 3.4.0.
*/
pub const SDL_HINT_JOYSTICK_HIDAPI_SINPUT: Hint = Hint { name: "SDL_HINT_JOYSTICK_HIDAPI_SINPUT", value: "SDL_JOYSTICK_HIDAPI_SINPUT", doc: "/**\n* A variable controlling whether the HIDAPI driver for SInput controllers\n* should be used.\n*\n* More info - https://github.com/HandHeldLegend/SInput-HID\n*\n* The variable can be set to the following values:\n*\n* - \"0\": HIDAPI driver is not used.\n* - \"1\": HIDAPI driver is used.\n*\n* The default is the value of SDL_HINT_JOYSTICK_HIDAPI.\n*\n* This hint should be set before initializing joysticks and gamepads.\n*\n* \\since This hint is available since SDL 3.4.0.\n*/\n" };
/**
* A variable controlling whether the HIDAPI driver for ZUIKI controllers
* should be used.
*
* The variable can be set to the following values:
*
* - "0": HIDAPI driver is not used.
* - "1": HIDAPI driver is used.
*
* The default is the value of SDL_HINT_JOYSTICK_HIDAPI.
*
* This hint should be set before initializing joysticks and gamepads.
*
* \since This hint is available since SDL 3.4.0.
*/
pub const SDL_HINT_JOYSTICK_HIDAPI_ZUIKI: Hint = Hint { name: "SDL_HINT_JOYSTICK_HIDAPI_ZUIKI", value: "SDL_JOYSTICK_HIDAPI_ZUIKI", doc: "/**\n* A variable controlling whether the HIDAPI driver for ZUIKI controllers\n* should be used.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": HIDAPI driver is not used.\n* - \"1\": HIDAPI driver is used.\n*\n* The default is the value of SDL_HINT_JOYSTICK_HIDAPI.\n*\n* This hint should be set before initializing joysticks and gamepads.\n*\n* \\since This hint is available since SDL 3.4.0.\n*/\n" };
/**
* A variable controlling whether the HIDAPI driver for Flydigi controllers
* should be used.
*
* The variable can be set to the following values:
*
* - "0": HIDAPI driver is not used.
* - "1": HIDAPI driver is used.
*
* The default is the value of SDL_HINT_JOYSTICK_HIDAPI.
*
* This hint should be set before initializing joysticks and gamepads.
*
* \since This hint is available since SDL 3.4.0.
*/
pub const SDL_HINT_JOYSTICK_HIDAPI_FLYDIGI: Hint = Hint { name: "SDL_HINT_JOYSTICK_HIDAPI_FLYDIGI", value: "SDL_JOYSTICK_HIDAPI_FLYDIGI", doc: "/**\n* A variable controlling whether the HIDAPI driver for Flydigi controllers\n* should be used.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": HIDAPI driver is not used.\n* - \"1\": HIDAPI driver is used.\n*\n* The default is the value of SDL_HINT_JOYSTICK_HIDAPI.\n*\n* This hint should be set before initializing joysticks and gamepads.\n*\n* \\since This hint is available since SDL 3.4.0.\n*/\n" };
/**
* A variable controlling whether the HIDAPI driver for GameSir controllers
* should be used.
*
* The variable can be set to the following values:
*
* - "0": HIDAPI driver is not used.
* - "1": HIDAPI driver is used.
*
* The default is the value of SDL_HINT_JOYSTICK_HIDAPI.
*
* This hint should be set before initializing joysticks and gamepads.
*
* \since This hint is available since SDL 3.5.0.
*/
pub const SDL_HINT_JOYSTICK_HIDAPI_GAMESIR: Hint = Hint { name: "SDL_HINT_JOYSTICK_HIDAPI_GAMESIR", value: "SDL_JOYSTICK_HIDAPI_GAMESIR", doc: "/**\n* A variable controlling whether the HIDAPI driver for GameSir controllers\n* should be used.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": HIDAPI driver is not used.\n* - \"1\": HIDAPI driver is used.\n*\n* The default is the value of SDL_HINT_JOYSTICK_HIDAPI.\n*\n* This hint should be set before initializing joysticks and gamepads.\n*\n* \\since This hint is available since SDL 3.5.0.\n*/\n" };
/**
* A variable controlling whether the HIDAPI driver for Nintendo Switch
* controllers should be used.
*
* The variable can be set to the following values:
*
* - "0": HIDAPI driver is not used.
* - "1": HIDAPI driver is used.
*
* The default is the value of SDL_HINT_JOYSTICK_HIDAPI.
*
* This hint should be set before initializing joysticks and gamepads.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_HIDAPI_SWITCH: Hint = Hint { name: "SDL_HINT_JOYSTICK_HIDAPI_SWITCH", value: "SDL_JOYSTICK_HIDAPI_SWITCH", doc: "/**\n* A variable controlling whether the HIDAPI driver for Nintendo Switch\n* controllers should be used.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": HIDAPI driver is not used.\n* - \"1\": HIDAPI driver is used.\n*\n* The default is the value of SDL_HINT_JOYSTICK_HIDAPI.\n*\n* This hint should be set before initializing joysticks and gamepads.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the Home button LED should be turned on when
* a Nintendo Switch Pro controller is opened.
*
* The variable can be set to the following values:
*
* - "0": Home button LED is turned off.
* - "1": Home button LED is turned on.
*
* By default the Home button LED state is not changed. This hint can also be
* set to a floating point value between 0.0 and 1.0 which controls the
* brightness of the Home button LED.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_HIDAPI_SWITCH_HOME_LED: Hint = Hint { name: "SDL_HINT_JOYSTICK_HIDAPI_SWITCH_HOME_LED", value: "SDL_JOYSTICK_HIDAPI_SWITCH_HOME_LED", doc: "/**\n* A variable controlling whether the Home button LED should be turned on when\n* a Nintendo Switch Pro controller is opened.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Home button LED is turned off.\n* - \"1\": Home button LED is turned on.\n*\n* By default the Home button LED state is not changed. This hint can also be\n* set to a floating point value between 0.0 and 1.0 which controls the\n* brightness of the Home button LED.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the player LEDs should be lit to indicate
* which player is associated with a Nintendo Switch controller.
*
* The variable can be set to the following values:
*
* - "0": Player LEDs are not enabled.
* - "1": Player LEDs are enabled. (default)
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_HIDAPI_SWITCH_PLAYER_LED: Hint = Hint { name: "SDL_HINT_JOYSTICK_HIDAPI_SWITCH_PLAYER_LED", value: "SDL_JOYSTICK_HIDAPI_SWITCH_PLAYER_LED", doc: "/**\n* A variable controlling whether the player LEDs should be lit to indicate\n* which player is associated with a Nintendo Switch controller.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Player LEDs are not enabled.\n* - \"1\": Player LEDs are enabled. (default)\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the HIDAPI driver for Nintendo Switch 2
* controllers should be used.
*
* The variable can be set to the following values:
*
* - "0": HIDAPI driver is not used.
* - "1": HIDAPI driver is used.
*
* The default is the value of SDL_HINT_JOYSTICK_HIDAPI.
*
* This hint should be set before initializing joysticks and gamepads.
*
* \since This hint is available since SDL 3.4.0.
*/
pub const SDL_HINT_JOYSTICK_HIDAPI_SWITCH2: Hint = Hint { name: "SDL_HINT_JOYSTICK_HIDAPI_SWITCH2", value: "SDL_JOYSTICK_HIDAPI_SWITCH2", doc: "/**\n* A variable controlling whether the HIDAPI driver for Nintendo Switch 2\n* controllers should be used.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": HIDAPI driver is not used.\n* - \"1\": HIDAPI driver is used.\n*\n* The default is the value of SDL_HINT_JOYSTICK_HIDAPI.\n*\n* This hint should be set before initializing joysticks and gamepads.\n*\n* \\since This hint is available since SDL 3.4.0.\n*/\n" };
/**
* A variable controlling whether Nintendo Switch Joy-Con controllers will be
* in vertical mode when using the HIDAPI driver.
*
* The variable can be set to the following values:
*
* - "0": Left and right Joy-Con controllers will not be in vertical mode.
*   (default)
* - "1": Left and right Joy-Con controllers will be in vertical mode.
*
* This hint should be set before opening a Joy-Con controller.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_HIDAPI_VERTICAL_JOY_CONS: Hint = Hint { name: "SDL_HINT_JOYSTICK_HIDAPI_VERTICAL_JOY_CONS", value: "SDL_JOYSTICK_HIDAPI_VERTICAL_JOY_CONS", doc: "/**\n* A variable controlling whether Nintendo Switch Joy-Con controllers will be\n* in vertical mode when using the HIDAPI driver.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Left and right Joy-Con controllers will not be in vertical mode.\n*   (default)\n* - \"1\": Left and right Joy-Con controllers will be in vertical mode.\n*\n* This hint should be set before opening a Joy-Con controller.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the HIDAPI driver for Nintendo Wii and Wii U
* controllers should be used.
*
* The variable can be set to the following values:
*
* - "0": HIDAPI driver is not used.
* - "1": HIDAPI driver is used.
*
* This driver doesn't work with the dolphinbar, so the default is false for
* now.
*
* This hint should be set before initializing joysticks and gamepads.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_HIDAPI_WII: Hint = Hint { name: "SDL_HINT_JOYSTICK_HIDAPI_WII", value: "SDL_JOYSTICK_HIDAPI_WII", doc: "/**\n* A variable controlling whether the HIDAPI driver for Nintendo Wii and Wii U\n* controllers should be used.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": HIDAPI driver is not used.\n* - \"1\": HIDAPI driver is used.\n*\n* This driver doesn't work with the dolphinbar, so the default is false for\n* now.\n*\n* This hint should be set before initializing joysticks and gamepads.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the player LEDs should be lit to indicate
* which player is associated with a Wii controller.
*
* The variable can be set to the following values:
*
* - "0": Player LEDs are not enabled.
* - "1": Player LEDs are enabled. (default)
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_HIDAPI_WII_PLAYER_LED: Hint = Hint { name: "SDL_HINT_JOYSTICK_HIDAPI_WII_PLAYER_LED", value: "SDL_JOYSTICK_HIDAPI_WII_PLAYER_LED", doc: "/**\n* A variable controlling whether the player LEDs should be lit to indicate\n* which player is associated with a Wii controller.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Player LEDs are not enabled.\n* - \"1\": Player LEDs are enabled. (default)\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the HIDAPI driver for XBox controllers
* should be used.
*
* The variable can be set to the following values:
*
* - "0": HIDAPI driver is not used.
* - "1": HIDAPI driver is used.
*
* The default is "0" on Windows, otherwise the value of
* SDL_HINT_JOYSTICK_HIDAPI
*
* This hint should be set before initializing joysticks and gamepads.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_HIDAPI_XBOX: Hint = Hint { name: "SDL_HINT_JOYSTICK_HIDAPI_XBOX", value: "SDL_JOYSTICK_HIDAPI_XBOX", doc: "/**\n* A variable controlling whether the HIDAPI driver for XBox controllers\n* should be used.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": HIDAPI driver is not used.\n* - \"1\": HIDAPI driver is used.\n*\n* The default is \"0\" on Windows, otherwise the value of\n* SDL_HINT_JOYSTICK_HIDAPI\n*\n* This hint should be set before initializing joysticks and gamepads.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the HIDAPI driver for XBox 360 controllers
* should be used.
*
* The variable can be set to the following values:
*
* - "0": HIDAPI driver is not used.
* - "1": HIDAPI driver is used.
*
* The default is the value of SDL_HINT_JOYSTICK_HIDAPI_XBOX
*
* This hint should be set before initializing joysticks and gamepads.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_HIDAPI_XBOX_360: Hint = Hint { name: "SDL_HINT_JOYSTICK_HIDAPI_XBOX_360", value: "SDL_JOYSTICK_HIDAPI_XBOX_360", doc: "/**\n* A variable controlling whether the HIDAPI driver for XBox 360 controllers\n* should be used.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": HIDAPI driver is not used.\n* - \"1\": HIDAPI driver is used.\n*\n* The default is the value of SDL_HINT_JOYSTICK_HIDAPI_XBOX\n*\n* This hint should be set before initializing joysticks and gamepads.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the player LEDs should be lit to indicate
* which player is associated with an Xbox 360 controller.
*
* The variable can be set to the following values:
*
* - "0": Player LEDs are not enabled.
* - "1": Player LEDs are enabled. (default)
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_HIDAPI_XBOX_360_PLAYER_LED: Hint = Hint { name: "SDL_HINT_JOYSTICK_HIDAPI_XBOX_360_PLAYER_LED", value: "SDL_JOYSTICK_HIDAPI_XBOX_360_PLAYER_LED", doc: "/**\n* A variable controlling whether the player LEDs should be lit to indicate\n* which player is associated with an Xbox 360 controller.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Player LEDs are not enabled.\n* - \"1\": Player LEDs are enabled. (default)\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the HIDAPI driver for XBox 360 wireless
* controllers should be used.
*
* The variable can be set to the following values:
*
* - "0": HIDAPI driver is not used.
* - "1": HIDAPI driver is used.
*
* The default is the value of SDL_HINT_JOYSTICK_HIDAPI_XBOX_360
*
* This hint should be set before initializing joysticks and gamepads.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_HIDAPI_XBOX_360_WIRELESS: Hint = Hint { name: "SDL_HINT_JOYSTICK_HIDAPI_XBOX_360_WIRELESS", value: "SDL_JOYSTICK_HIDAPI_XBOX_360_WIRELESS", doc: "/**\n* A variable controlling whether the HIDAPI driver for XBox 360 wireless\n* controllers should be used.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": HIDAPI driver is not used.\n* - \"1\": HIDAPI driver is used.\n*\n* The default is the value of SDL_HINT_JOYSTICK_HIDAPI_XBOX_360\n*\n* This hint should be set before initializing joysticks and gamepads.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the HIDAPI driver for XBox One controllers
* should be used.
*
* The variable can be set to the following values:
*
* - "0": HIDAPI driver is not used.
* - "1": HIDAPI driver is used.
*
* The default is the value of SDL_HINT_JOYSTICK_HIDAPI_XBOX.
*
* This hint should be set before initializing joysticks and gamepads.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_HIDAPI_XBOX_ONE: Hint = Hint { name: "SDL_HINT_JOYSTICK_HIDAPI_XBOX_ONE", value: "SDL_JOYSTICK_HIDAPI_XBOX_ONE", doc: "/**\n* A variable controlling whether the HIDAPI driver for XBox One controllers\n* should be used.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": HIDAPI driver is not used.\n* - \"1\": HIDAPI driver is used.\n*\n* The default is the value of SDL_HINT_JOYSTICK_HIDAPI_XBOX.\n*\n* This hint should be set before initializing joysticks and gamepads.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the Home button LED should be turned on when
* an Xbox One controller is opened.
*
* The variable can be set to the following values:
*
* - "0": Home button LED is turned off.
* - "1": Home button LED is turned on.
*
* By default the Home button LED state is not changed. This hint can also be
* set to a floating point value between 0.0 and 1.0 which controls the
* brightness of the Home button LED. The default brightness is 0.4.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_HIDAPI_XBOX_ONE_HOME_LED: Hint = Hint { name: "SDL_HINT_JOYSTICK_HIDAPI_XBOX_ONE_HOME_LED", value: "SDL_JOYSTICK_HIDAPI_XBOX_ONE_HOME_LED", doc: "/**\n* A variable controlling whether the Home button LED should be turned on when\n* an Xbox One controller is opened.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Home button LED is turned off.\n* - \"1\": Home button LED is turned on.\n*\n* By default the Home button LED state is not changed. This hint can also be\n* set to a floating point value between 0.0 and 1.0 which controls the\n* brightness of the Home button LED. The default brightness is 0.4.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the new HIDAPI driver for wired Xbox One
* (GIP) controllers should be used.
*
* The variable can be set to the following values:
*
* - "0": HIDAPI driver is not used.
* - "1": HIDAPI driver is used.
*
* The default is the value of SDL_HINT_JOYSTICK_HIDAPI_XBOX_ONE.
*
* This hint should be set before initializing joysticks and gamepads.
*
* \since This hint is available since SDL 3.4.0.
*/
pub const SDL_HINT_JOYSTICK_HIDAPI_GIP: Hint = Hint { name: "SDL_HINT_JOYSTICK_HIDAPI_GIP", value: "SDL_JOYSTICK_HIDAPI_GIP", doc: "/**\n* A variable controlling whether the new HIDAPI driver for wired Xbox One\n* (GIP) controllers should be used.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": HIDAPI driver is not used.\n* - \"1\": HIDAPI driver is used.\n*\n* The default is the value of SDL_HINT_JOYSTICK_HIDAPI_XBOX_ONE.\n*\n* This hint should be set before initializing joysticks and gamepads.\n*\n* \\since This hint is available since SDL 3.4.0.\n*/\n" };
/**
* A variable controlling whether the new HIDAPI driver for wired Xbox One
* (GIP) controllers should reset the controller if it can't get the metadata
* from the controller.
*
* The variable can be set to the following values:
*
* - "0": Assume this is a generic controller.
* - "1": Reset the controller to get metadata.
*
* By default the controller is not reset.
*
* This hint should be set before initializing joysticks and gamepads.
*
* \since This hint is available since SDL 3.4.0.
*/
pub const SDL_HINT_JOYSTICK_HIDAPI_GIP_RESET_FOR_METADATA: Hint = Hint { name: "SDL_HINT_JOYSTICK_HIDAPI_GIP_RESET_FOR_METADATA", value: "SDL_JOYSTICK_HIDAPI_GIP_RESET_FOR_METADATA", doc: "/**\n* A variable controlling whether the new HIDAPI driver for wired Xbox One\n* (GIP) controllers should reset the controller if it can't get the metadata\n* from the controller.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Assume this is a generic controller.\n* - \"1\": Reset the controller to get metadata.\n*\n* By default the controller is not reset.\n*\n* This hint should be set before initializing joysticks and gamepads.\n*\n* \\since This hint is available since SDL 3.4.0.\n*/\n" };
/**
* A variable controlling whether IOKit should be used for controller
* handling.
*
* The variable can be set to the following values:
*
* - "0": IOKit is not used.
* - "1": IOKit is used. (default)
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_IOKIT: Hint = Hint { name: "SDL_HINT_JOYSTICK_IOKIT", value: "SDL_JOYSTICK_IOKIT", doc: "/**\n* A variable controlling whether IOKit should be used for controller\n* handling.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": IOKit is not used.\n* - \"1\": IOKit is used. (default)\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether to use the classic /dev/input/js* joystick
* interface or the newer /dev/input/event* joystick interface on Linux.
*
* The variable can be set to the following values:
*
* - "0": Use /dev/input/event* (default)
* - "1": Use /dev/input/js*
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_LINUX_CLASSIC: Hint = Hint { name: "SDL_HINT_JOYSTICK_LINUX_CLASSIC", value: "SDL_JOYSTICK_LINUX_CLASSIC", doc: "/**\n* A variable controlling whether to use the classic /dev/input/js* joystick\n* interface or the newer /dev/input/event* joystick interface on Linux.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Use /dev/input/event* (default)\n* - \"1\": Use /dev/input/js*\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether joysticks on Linux adhere to their
* HID-defined deadzones or return unfiltered values.
*
* The variable can be set to the following values:
*
* - "0": Return unfiltered joystick axis values. (default)
* - "1": Return axis values with deadzones taken into account.
*
* This hint should be set before a controller is opened.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_LINUX_DEADZONES: Hint = Hint { name: "SDL_HINT_JOYSTICK_LINUX_DEADZONES", value: "SDL_JOYSTICK_LINUX_DEADZONES", doc: "/**\n* A variable controlling whether joysticks on Linux adhere to their\n* HID-defined deadzones or return unfiltered values.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Return unfiltered joystick axis values. (default)\n* - \"1\": Return axis values with deadzones taken into account.\n*\n* This hint should be set before a controller is opened.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether joysticks on Linux will always treat 'hat'
* axis inputs (ABS_HAT0X - ABS_HAT3Y) as 8-way digital hats without checking
* whether they may be analog.
*
* The variable can be set to the following values:
*
* - "0": Only map hat axis inputs to digital hat outputs if the input axes
*   appear to actually be digital. (default)
* - "1": Always handle the input axes numbered ABS_HAT0X to ABS_HAT3Y as
*   digital hats.
*
* This hint should be set before a controller is opened.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_LINUX_DIGITAL_HATS: Hint = Hint { name: "SDL_HINT_JOYSTICK_LINUX_DIGITAL_HATS", value: "SDL_JOYSTICK_LINUX_DIGITAL_HATS", doc: "/**\n* A variable controlling whether joysticks on Linux will always treat 'hat'\n* axis inputs (ABS_HAT0X - ABS_HAT3Y) as 8-way digital hats without checking\n* whether they may be analog.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Only map hat axis inputs to digital hat outputs if the input axes\n*   appear to actually be digital. (default)\n* - \"1\": Always handle the input axes numbered ABS_HAT0X to ABS_HAT3Y as\n*   digital hats.\n*\n* This hint should be set before a controller is opened.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether digital hats on Linux will apply deadzones
* to their underlying input axes or use unfiltered values.
*
* The variable can be set to the following values:
*
* - "0": Return digital hat values based on unfiltered input axis values.
* - "1": Return digital hat values with deadzones on the input axes taken
*   into account. (default)
*
* This hint should be set before a controller is opened.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_LINUX_HAT_DEADZONES: Hint = Hint { name: "SDL_HINT_JOYSTICK_LINUX_HAT_DEADZONES", value: "SDL_JOYSTICK_LINUX_HAT_DEADZONES", doc: "/**\n* A variable controlling whether digital hats on Linux will apply deadzones\n* to their underlying input axes or use unfiltered values.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Return digital hat values based on unfiltered input axis values.\n* - \"1\": Return digital hat values with deadzones on the input axes taken\n*   into account. (default)\n*\n* This hint should be set before a controller is opened.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether GCController should be used for controller
* handling.
*
* The variable can be set to the following values:
*
* - "0": GCController is not used.
* - "1": GCController is used. (default)
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_MFI: Hint = Hint { name: "SDL_HINT_JOYSTICK_MFI", value: "SDL_JOYSTICK_MFI", doc: "/**\n* A variable controlling whether GCController should be used for controller\n* handling.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": GCController is not used.\n* - \"1\": GCController is used. (default)\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the RAWINPUT joystick drivers should be used
* for better handling XInput-capable devices.
*
* The variable can be set to the following values:
*
* - "0": RAWINPUT drivers are not used. (default)
* - "1": RAWINPUT drivers are used.
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_RAWINPUT: Hint = Hint { name: "SDL_HINT_JOYSTICK_RAWINPUT", value: "SDL_JOYSTICK_RAWINPUT", doc: "/**\n* A variable controlling whether the RAWINPUT joystick drivers should be used\n* for better handling XInput-capable devices.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": RAWINPUT drivers are not used. (default)\n* - \"1\": RAWINPUT drivers are used.\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the RAWINPUT driver should pull correlated
* data from XInput.
*
* The variable can be set to the following values:
*
* - "0": RAWINPUT driver will only use data from raw input APIs.
* - "1": RAWINPUT driver will also pull data from XInput and
*   Windows.Gaming.Input, providing better trigger axes, guide button
*   presses, and rumble support for Xbox controllers. (default)
*
* This hint should be set before a gamepad is opened.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_RAWINPUT_CORRELATE_XINPUT: Hint = Hint { name: "SDL_HINT_JOYSTICK_RAWINPUT_CORRELATE_XINPUT", value: "SDL_JOYSTICK_RAWINPUT_CORRELATE_XINPUT", doc: "/**\n* A variable controlling whether the RAWINPUT driver should pull correlated\n* data from XInput.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": RAWINPUT driver will only use data from raw input APIs.\n* - \"1\": RAWINPUT driver will also pull data from XInput and\n*   Windows.Gaming.Input, providing better trigger axes, guide button\n*   presses, and rumble support for Xbox controllers. (default)\n*\n* This hint should be set before a gamepad is opened.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the ROG Chakram mice should show up as
* joysticks.
*
* The variable can be set to the following values:
*
* - "0": ROG Chakram mice do not show up as joysticks. (default)
* - "1": ROG Chakram mice show up as joysticks.
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_ROG_CHAKRAM: Hint = Hint { name: "SDL_HINT_JOYSTICK_ROG_CHAKRAM", value: "SDL_JOYSTICK_ROG_CHAKRAM", doc: "/**\n* A variable controlling whether the ROG Chakram mice should show up as\n* joysticks.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": ROG Chakram mice do not show up as joysticks. (default)\n* - \"1\": ROG Chakram mice show up as joysticks.\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether a separate thread should be used for
* handling joystick detection and raw input messages on Windows.
*
* The variable can be set to the following values:
*
* - "0": A separate thread is not used.
* - "1": A separate thread is used for handling raw input messages. (default)
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_THREAD: Hint = Hint { name: "SDL_HINT_JOYSTICK_THREAD", value: "SDL_JOYSTICK_THREAD", doc: "/**\n* A variable controlling whether a separate thread should be used for\n* handling joystick detection and raw input messages on Windows.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": A separate thread is not used.\n* - \"1\": A separate thread is used for handling raw input messages. (default)\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable containing a list of throttle style controllers.
*
* The format of the string is a comma separated list of USB VID/PID pairs in
* hexadecimal form, e.g.
*
* `0xAAAA/0xBBBB,0xCCCC/0xDDDD`
*
* The variable can also take the form of "@file", in which case the named
* file will be loaded and interpreted as the value of the variable.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_THROTTLE_DEVICES: Hint = Hint { name: "SDL_HINT_JOYSTICK_THROTTLE_DEVICES", value: "SDL_JOYSTICK_THROTTLE_DEVICES", doc: "/**\n* A variable containing a list of throttle style controllers.\n*\n* The format of the string is a comma separated list of USB VID/PID pairs in\n* hexadecimal form, e.g.\n*\n* `0xAAAA/0xBBBB,0xCCCC/0xDDDD`\n*\n* The variable can also take the form of \"@file\", in which case the named\n* file will be loaded and interpreted as the value of the variable.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable containing a list of devices that are not throttle style
* controllers.
*
* This will override SDL_HINT_JOYSTICK_THROTTLE_DEVICES and the built in
* device list.
*
* The format of the string is a comma separated list of USB VID/PID pairs in
* hexadecimal form, e.g.
*
* `0xAAAA/0xBBBB,0xCCCC/0xDDDD`
*
* The variable can also take the form of "@file", in which case the named
* file will be loaded and interpreted as the value of the variable.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_THROTTLE_DEVICES_EXCLUDED: Hint = Hint { name: "SDL_HINT_JOYSTICK_THROTTLE_DEVICES_EXCLUDED", value: "SDL_JOYSTICK_THROTTLE_DEVICES_EXCLUDED", doc: "/**\n* A variable containing a list of devices that are not throttle style\n* controllers.\n*\n* This will override SDL_HINT_JOYSTICK_THROTTLE_DEVICES and the built in\n* device list.\n*\n* The format of the string is a comma separated list of USB VID/PID pairs in\n* hexadecimal form, e.g.\n*\n* `0xAAAA/0xBBBB,0xCCCC/0xDDDD`\n*\n* The variable can also take the form of \"@file\", in which case the named\n* file will be loaded and interpreted as the value of the variable.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether Windows.Gaming.Input should be used for
* controller handling.
*
* The variable can be set to the following values:
*
* - "0": WGI is not used. (default)
* - "1": WGI is used.
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_WGI: Hint = Hint { name: "SDL_HINT_JOYSTICK_WGI", value: "SDL_JOYSTICK_WGI", doc: "/**\n* A variable controlling whether Windows.Gaming.Input should be used for\n* controller handling.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": WGI is not used. (default)\n* - \"1\": WGI is used.\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable containing a list of wheel style controllers.
*
* The format of the string is a comma separated list of USB VID/PID pairs in
* hexadecimal form, e.g.
*
* `0xAAAA/0xBBBB,0xCCCC/0xDDDD`
*
* The variable can also take the form of "@file", in which case the named
* file will be loaded and interpreted as the value of the variable.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_WHEEL_DEVICES: Hint = Hint { name: "SDL_HINT_JOYSTICK_WHEEL_DEVICES", value: "SDL_JOYSTICK_WHEEL_DEVICES", doc: "/**\n* A variable containing a list of wheel style controllers.\n*\n* The format of the string is a comma separated list of USB VID/PID pairs in\n* hexadecimal form, e.g.\n*\n* `0xAAAA/0xBBBB,0xCCCC/0xDDDD`\n*\n* The variable can also take the form of \"@file\", in which case the named\n* file will be loaded and interpreted as the value of the variable.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable containing a list of devices that are not wheel style
* controllers.
*
* This will override SDL_HINT_JOYSTICK_WHEEL_DEVICES and the built in device
* list.
*
* The format of the string is a comma separated list of USB VID/PID pairs in
* hexadecimal form, e.g.
*
* `0xAAAA/0xBBBB,0xCCCC/0xDDDD`
*
* The variable can also take the form of "@file", in which case the named
* file will be loaded and interpreted as the value of the variable.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_WHEEL_DEVICES_EXCLUDED: Hint = Hint { name: "SDL_HINT_JOYSTICK_WHEEL_DEVICES_EXCLUDED", value: "SDL_JOYSTICK_WHEEL_DEVICES_EXCLUDED", doc: "/**\n* A variable containing a list of devices that are not wheel style\n* controllers.\n*\n* This will override SDL_HINT_JOYSTICK_WHEEL_DEVICES and the built in device\n* list.\n*\n* The format of the string is a comma separated list of USB VID/PID pairs in\n* hexadecimal form, e.g.\n*\n* `0xAAAA/0xBBBB,0xCCCC/0xDDDD`\n*\n* The variable can also take the form of \"@file\", in which case the named\n* file will be loaded and interpreted as the value of the variable.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable containing a list of devices known to have all axes centered at
* zero.
*
* The format of the string is a comma separated list of USB VID/PID pairs in
* hexadecimal form, e.g.
*
* `0xAAAA/0xBBBB,0xCCCC/0xDDDD`
*
* The variable can also take the form of "@file", in which case the named
* file will be loaded and interpreted as the value of the variable.
*
* This hint should be set before a controller is opened.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_JOYSTICK_ZERO_CENTERED_DEVICES: Hint = Hint { name: "SDL_HINT_JOYSTICK_ZERO_CENTERED_DEVICES", value: "SDL_JOYSTICK_ZERO_CENTERED_DEVICES", doc: "/**\n* A variable containing a list of devices known to have all axes centered at\n* zero.\n*\n* The format of the string is a comma separated list of USB VID/PID pairs in\n* hexadecimal form, e.g.\n*\n* `0xAAAA/0xBBBB,0xCCCC/0xDDDD`\n*\n* The variable can also take the form of \"@file\", in which case the named\n* file will be loaded and interpreted as the value of the variable.\n*\n* This hint should be set before a controller is opened.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable containing a list of devices and their desired number of haptic
* (force feedback) enabled axis.
*
* The format of the string is a comma separated list of USB VID/PID pairs in
* hexadecimal form plus the number of desired axes, e.g.
*
* `0xAAAA/0xBBBB/1,0xCCCC/0xDDDD/3`
*
* This hint supports a "wildcard" device that will set the number of haptic
* axes on all initialized haptic devices which were not defined explicitly in
* this hint.
*
* `0xFFFF/0xFFFF/1`
*
* This hint should be set before a controller is opened. The number of haptic
* axes won't exceed the number of real axes found on the device.
*
* \since This hint is available since SDL 3.2.5.
*/
pub const SDL_HINT_JOYSTICK_HAPTIC_AXES: Hint = Hint { name: "SDL_HINT_JOYSTICK_HAPTIC_AXES", value: "SDL_JOYSTICK_HAPTIC_AXES", doc: "/**\n* A variable containing a list of devices and their desired number of haptic\n* (force feedback) enabled axis.\n*\n* The format of the string is a comma separated list of USB VID/PID pairs in\n* hexadecimal form plus the number of desired axes, e.g.\n*\n* `0xAAAA/0xBBBB/1,0xCCCC/0xDDDD/3`\n*\n* This hint supports a \"wildcard\" device that will set the number of haptic\n* axes on all initialized haptic devices which were not defined explicitly in\n* this hint.\n*\n* `0xFFFF/0xFFFF/1`\n*\n* This hint should be set before a controller is opened. The number of haptic\n* axes won't exceed the number of real axes found on the device.\n*\n* \\since This hint is available since SDL 3.2.5.\n*/\n" };
/**
* A variable that controls keycode representation in keyboard events.
*
* This variable is a comma separated set of options for translating keycodes
* in events:
*
* - "none": Keycode options are cleared, this overrides other options.
* - "hide_numpad": The numpad keysyms will be translated into their
*   non-numpad versions based on the current NumLock state. For example,
*   SDLK_KP_4 would become SDLK_4 if SDL_KMOD_NUM is set in the event
*   modifiers, and SDLK_LEFT if it is unset.
* - "french_numbers": The number row on French keyboards is inverted, so
*   pressing the 1 key would yield the keycode SDLK_1, or '1', instead of
*   SDLK_AMPERSAND, or '&'
* - "latin_letters": For keyboards using non-Latin letters, such as Russian
*   or Thai, the letter keys generate keycodes as though it had an English
*   QWERTY layout. e.g. pressing the key associated with SDL_SCANCODE_A on a
*   Russian keyboard would yield 'a' instead of a Cyrillic letter.
*
* The default value for this hint is "french_numbers,latin_letters"
*
* Some platforms like Emscripten only provide modified keycodes and the
* options are not used.
*
* These options do not affect the return value of SDL_GetKeyFromScancode() or
* SDL_GetScancodeFromKey(), they just apply to the keycode included in key
* events.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_KEYCODE_OPTIONS: Hint = Hint { name: "SDL_HINT_KEYCODE_OPTIONS", value: "SDL_KEYCODE_OPTIONS", doc: "/**\n* A variable that controls keycode representation in keyboard events.\n*\n* This variable is a comma separated set of options for translating keycodes\n* in events:\n*\n* - \"none\": Keycode options are cleared, this overrides other options.\n* - \"hide_numpad\": The numpad keysyms will be translated into their\n*   non-numpad versions based on the current NumLock state. For example,\n*   SDLK_KP_4 would become SDLK_4 if SDL_KMOD_NUM is set in the event\n*   modifiers, and SDLK_LEFT if it is unset.\n* - \"french_numbers\": The number row on French keyboards is inverted, so\n*   pressing the 1 key would yield the keycode SDLK_1, or '1', instead of\n*   SDLK_AMPERSAND, or '&'\n* - \"latin_letters\": For keyboards using non-Latin letters, such as Russian\n*   or Thai, the letter keys generate keycodes as though it had an English\n*   QWERTY layout. e.g. pressing the key associated with SDL_SCANCODE_A on a\n*   Russian keyboard would yield 'a' instead of a Cyrillic letter.\n*\n* The default value for this hint is \"french_numbers,latin_letters\"\n*\n* Some platforms like Emscripten only provide modified keycodes and the\n* options are not used.\n*\n* These options do not affect the return value of SDL_GetKeyFromScancode() or\n* SDL_GetScancodeFromKey(), they just apply to the keycode included in key\n* events.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable that controls what KMSDRM device to use.
*
* SDL might open something like "/dev/dri/cardNN" to access KMSDRM
* functionality, where "NN" is a device index number. SDL makes a guess at
* the best index to use (usually zero), but the app or user can set this hint
* to a number between 0 and 99 to force selection.
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_KMSDRM_DEVICE_INDEX: Hint = Hint { name: "SDL_HINT_KMSDRM_DEVICE_INDEX", value: "SDL_KMSDRM_DEVICE_INDEX", doc: "/**\n* A variable that controls what KMSDRM device to use.\n*\n* SDL might open something like \"/dev/dri/cardNN\" to access KMSDRM\n* functionality, where \"NN\" is a device index number. SDL makes a guess at\n* the best index to use (usually zero), but the app or user can set this hint\n* to a number between 0 and 99 to force selection.\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable that controls whether SDL requires DRM master access in order to
* initialize the KMSDRM video backend.
*
* The DRM subsystem has a concept of a "DRM master" which is a DRM client
* that has the ability to set planes, set cursor, etc. When SDL is DRM
* master, it can draw to the screen using the SDL rendering APIs. Without DRM
* master, SDL is still able to process input and query attributes of attached
* displays, but it cannot change display state or draw to the screen
* directly.
*
* In some cases, it can be useful to have the KMSDRM backend even if it
* cannot be used for rendering. An app may want to use SDL for input
* processing while using another rendering API (such as an MMAL overlay on
* Raspberry Pi) or using its own code to render to DRM overlays that SDL
* doesn't support.
*
* The variable can be set to the following values:
*
* - "0": SDL will allow usage of the KMSDRM backend without DRM master.
* - "1": SDL Will require DRM master to use the KMSDRM backend. (default)
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_KMSDRM_REQUIRE_DRM_MASTER: Hint = Hint { name: "SDL_HINT_KMSDRM_REQUIRE_DRM_MASTER", value: "SDL_KMSDRM_REQUIRE_DRM_MASTER", doc: "/**\n* A variable that controls whether SDL requires DRM master access in order to\n* initialize the KMSDRM video backend.\n*\n* The DRM subsystem has a concept of a \"DRM master\" which is a DRM client\n* that has the ability to set planes, set cursor, etc. When SDL is DRM\n* master, it can draw to the screen using the SDL rendering APIs. Without DRM\n* master, SDL is still able to process input and query attributes of attached\n* displays, but it cannot change display state or draw to the screen\n* directly.\n*\n* In some cases, it can be useful to have the KMSDRM backend even if it\n* cannot be used for rendering. An app may want to use SDL for input\n* processing while using another rendering API (such as an MMAL overlay on\n* Raspberry Pi) or using its own code to render to DRM overlays that SDL\n* doesn't support.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": SDL will allow usage of the KMSDRM backend without DRM master.\n* - \"1\": SDL Will require DRM master to use the KMSDRM backend. (default)\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable that controls whether KMSDRM will use "atomic" functionality.
*
* The KMSDRM backend can use atomic commits, if both DRM_CLIENT_CAP_ATOMIC
* and DRM_CLIENT_CAP_UNIVERSAL_PLANES is supported by the system. As of SDL
* 3.4.0, it will favor this functionality, but in case this doesn't work well
* on a given system or other surprises, this hint can be used to disable it.
*
* This hint can not enable the functionality if it isn't available.
*
* The variable can be set to the following values:
*
* - "0": SDL will not use the KMSDRM "atomic" functionality.
* - "1": SDL will allow usage of the KMSDRM "atomic" functionality. (default)
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.4.0.
*/
pub const SDL_HINT_KMSDRM_ATOMIC: Hint = Hint { name: "SDL_HINT_KMSDRM_ATOMIC", value: "SDL_KMSDRM_ATOMIC", doc: "/**\n* A variable that controls whether KMSDRM will use \"atomic\" functionality.\n*\n* The KMSDRM backend can use atomic commits, if both DRM_CLIENT_CAP_ATOMIC\n* and DRM_CLIENT_CAP_UNIVERSAL_PLANES is supported by the system. As of SDL\n* 3.4.0, it will favor this functionality, but in case this doesn't work well\n* on a given system or other surprises, this hint can be used to disable it.\n*\n* This hint can not enable the functionality if it isn't available.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": SDL will not use the KMSDRM \"atomic\" functionality.\n* - \"1\": SDL will allow usage of the KMSDRM \"atomic\" functionality. (default)\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.4.0.\n*/\n" };
/**
* A variable controlling the default SDL log levels.
*
* This variable is a comma separated set of category=level tokens that define
* the default logging levels for SDL applications.
*
* The category can be a numeric category, one of "app", "error", "assert",
* "system", "audio", "video", "render", "input", "test", or `*` for any
* unspecified category.
*
* The level can be a numeric level, one of "verbose", "debug", "info",
* "warn", "error", "critical", or "quiet" to disable that category.
*
* You can omit the category if you want to set the logging level for all
* categories.
*
* If this hint isn't set, the default log levels are equivalent to:
*
* `app=info,assert=warn,test=verbose,*=error`
*
* If the `DEBUG_INVOCATION` environment variable is set to "1", the default
* log levels are equivalent to:
*
* `assert=warn,test=verbose,*=debug`
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_LOGGING: Hint = Hint { name: "SDL_HINT_LOGGING", value: "SDL_LOGGING", doc: "/**\n* A variable controlling the default SDL log levels.\n*\n* This variable is a comma separated set of category=level tokens that define\n* the default logging levels for SDL applications.\n*\n* The category can be a numeric category, one of \"app\", \"error\", \"assert\",\n* \"system\", \"audio\", \"video\", \"render\", \"input\", \"test\", or `*` for any\n* unspecified category.\n*\n* The level can be a numeric level, one of \"verbose\", \"debug\", \"info\",\n* \"warn\", \"error\", \"critical\", or \"quiet\" to disable that category.\n*\n* You can omit the category if you want to set the logging level for all\n* categories.\n*\n* If this hint isn't set, the default log levels are equivalent to:\n*\n* `app=info,assert=warn,test=verbose,*=error`\n*\n* If the `DEBUG_INVOCATION` environment variable is set to \"1\", the default\n* log levels are equivalent to:\n*\n* `assert=warn,test=verbose,*=debug`\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether to force the application to become the
* foreground process when launched on macOS.
*
* The variable can be set to the following values:
*
* - "0": The application is brought to the foreground when launched.
*   (default)
* - "1": The application may remain in the background when launched.
*
* This hint needs to be set before SDL_Init().
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_MAC_BACKGROUND_APP: Hint = Hint { name: "SDL_HINT_MAC_BACKGROUND_APP", value: "SDL_MAC_BACKGROUND_APP", doc: "/**\n* A variable controlling whether to force the application to become the\n* foreground process when launched on macOS.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": The application is brought to the foreground when launched.\n*   (default)\n* - \"1\": The application may remain in the background when launched.\n*\n* This hint needs to be set before SDL_Init().\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable that determines whether Ctrl+Click should generate a right-click
* event on macOS.
*
* The variable can be set to the following values:
*
* - "0": Ctrl+Click does not generate a right mouse button click event.
*   (default)
* - "1": Ctrl+Click generated a right mouse button click event.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_MAC_CTRL_CLICK_EMULATE_RIGHT_CLICK: Hint = Hint { name: "SDL_HINT_MAC_CTRL_CLICK_EMULATE_RIGHT_CLICK", value: "SDL_MAC_CTRL_CLICK_EMULATE_RIGHT_CLICK", doc: "/**\n* A variable that determines whether Ctrl+Click should generate a right-click\n* event on macOS.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Ctrl+Click does not generate a right mouse button click event.\n*   (default)\n* - \"1\": Ctrl+Click generated a right mouse button click event.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether dispatching OpenGL context updates should
* block the dispatching thread until the main thread finishes processing on
* macOS.
*
* The variable can be set to the following values:
*
* - "0": Dispatching OpenGL context updates will block the dispatching thread
*   until the main thread finishes processing. (default)
* - "1": Dispatching OpenGL context updates will allow the dispatching thread
*   to continue execution.
*
* Generally you want the default, but if you have OpenGL code in a background
* thread on a Mac, and the main thread hangs because it's waiting for that
* background thread, but that background thread is also hanging because it's
* waiting for the main thread to do an update, this might fix your issue.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_MAC_OPENGL_ASYNC_DISPATCH: Hint = Hint { name: "SDL_HINT_MAC_OPENGL_ASYNC_DISPATCH", value: "SDL_MAC_OPENGL_ASYNC_DISPATCH", doc: "/**\n* A variable controlling whether dispatching OpenGL context updates should\n* block the dispatching thread until the main thread finishes processing on\n* macOS.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Dispatching OpenGL context updates will block the dispatching thread\n*   until the main thread finishes processing. (default)\n* - \"1\": Dispatching OpenGL context updates will allow the dispatching thread\n*   to continue execution.\n*\n* Generally you want the default, but if you have OpenGL code in a background\n* thread on a Mac, and the main thread hangs because it's waiting for that\n* background thread, but that background thread is also hanging because it's\n* waiting for the main thread to do an update, this might fix your issue.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the Option key on macOS should be remapped
* to act as the Alt key.
*
* The variable can be set to the following values:
*
* - "none": The Option key is not remapped to Alt. (default)
* - "only_left": Only the left Option key is remapped to Alt.
* - "only_right": Only the right Option key is remapped to Alt.
* - "both": Both Option keys are remapped to Alt.
*
* This will prevent the triggering of key compositions that rely on the
* Option key, but will still send the Alt modifier for keyboard events. In
* the case that both Alt and Option are pressed, the Option key will be
* ignored. This is particularly useful for applications like terminal
* emulators and graphical user interfaces (GUIs) that rely on Alt key
* functionality for shortcuts or navigation. This does not apply to
* SDL_GetKeyFromScancode and only has an effect if IME is enabled.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_MAC_OPTION_AS_ALT: Hint = Hint { name: "SDL_HINT_MAC_OPTION_AS_ALT", value: "SDL_MAC_OPTION_AS_ALT", doc: "/**\n* A variable controlling whether the Option key on macOS should be remapped\n* to act as the Alt key.\n*\n* The variable can be set to the following values:\n*\n* - \"none\": The Option key is not remapped to Alt. (default)\n* - \"only_left\": Only the left Option key is remapped to Alt.\n* - \"only_right\": Only the right Option key is remapped to Alt.\n* - \"both\": Both Option keys are remapped to Alt.\n*\n* This will prevent the triggering of key compositions that rely on the\n* Option key, but will still send the Alt modifier for keyboard events. In\n* the case that both Alt and Option are pressed, the Option key will be\n* ignored. This is particularly useful for applications like terminal\n* emulators and graphical user interfaces (GUIs) that rely on Alt key\n* functionality for shortcuts or navigation. This does not apply to\n* SDL_GetKeyFromScancode and only has an effect if IME is enabled.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether SDL_EVENT_MOUSE_WHEEL event values will have
* momentum on macOS.
*
* The variable can be set to the following values:
*
* - "0": The mouse wheel events will have no momentum. (default)
* - "1": The mouse wheel events will have momentum.
*
* This hint needs to be set before SDL_Init().
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_MAC_SCROLL_MOMENTUM: Hint = Hint { name: "SDL_HINT_MAC_SCROLL_MOMENTUM", value: "SDL_MAC_SCROLL_MOMENTUM", doc: "/**\n* A variable controlling whether SDL_EVENT_MOUSE_WHEEL event values will have\n* momentum on macOS.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": The mouse wheel events will have no momentum. (default)\n* - \"1\": The mouse wheel events will have momentum.\n*\n* This hint needs to be set before SDL_Init().\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether holding down a key will repeat the pressed
* key or open the accents menu on macOS.
*
* The variable can be set to the following values:
*
* - "0": Holding a key will repeat the pressed key.
* - "1": Holding a key will open the accents menu for that key. (default)
*
* This hint needs to be set before SDL_Init().
*
* \since This hint is available since SDL 3.4.0.
*/
pub const SDL_HINT_MAC_PRESS_AND_HOLD: Hint = Hint { name: "SDL_HINT_MAC_PRESS_AND_HOLD", value: "SDL_MAC_PRESS_AND_HOLD", doc: "/**\n* A variable controlling whether holding down a key will repeat the pressed\n* key or open the accents menu on macOS.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Holding a key will repeat the pressed key.\n* - \"1\": Holding a key will open the accents menu for that key. (default)\n*\n* This hint needs to be set before SDL_Init().\n*\n* \\since This hint is available since SDL 3.4.0.\n*/\n" };
/**
* Request SDL_AppIterate() be called at a specific rate.
*
* If this is set to a number, it represents Hz, so "60" means try to iterate
* 60 times per second. "0" means to iterate as fast as possible. Negative
* values are illegal, but reserved, in case they are useful in a future
* revision of SDL.
*
* There are other strings that have special meaning. If set to "waitevent",
* SDL_AppIterate will not be called until new event(s) have arrived (and been
* processed by SDL_AppEvent). This can be useful for apps that are completely
* idle except in response to input.
*
* On some platforms, or if you are using SDL_main instead of SDL_AppIterate,
* this hint is ignored. When the hint can be used, it is allowed to be
* changed at any time.
*
* This defaults to 0, and specifying NULL for the hint's value will restore
* the default.
*
* This doesn't have to be an integer value. For example, "59.94" won't be
* rounded to an integer rate; the digits after the decimal are actually
* respected.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_MAIN_CALLBACK_RATE: Hint = Hint { name: "SDL_HINT_MAIN_CALLBACK_RATE", value: "SDL_MAIN_CALLBACK_RATE", doc: "/**\n* Request SDL_AppIterate() be called at a specific rate.\n*\n* If this is set to a number, it represents Hz, so \"60\" means try to iterate\n* 60 times per second. \"0\" means to iterate as fast as possible. Negative\n* values are illegal, but reserved, in case they are useful in a future\n* revision of SDL.\n*\n* There are other strings that have special meaning. If set to \"waitevent\",\n* SDL_AppIterate will not be called until new event(s) have arrived (and been\n* processed by SDL_AppEvent). This can be useful for apps that are completely\n* idle except in response to input.\n*\n* On some platforms, or if you are using SDL_main instead of SDL_AppIterate,\n* this hint is ignored. When the hint can be used, it is allowed to be\n* changed at any time.\n*\n* This defaults to 0, and specifying NULL for the hint's value will restore\n* the default.\n*\n* This doesn't have to be an integer value. For example, \"59.94\" won't be\n* rounded to an integer rate; the digits after the decimal are actually\n* respected.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the mouse is captured while mouse buttons
* are pressed.
*
* The variable can be set to the following values:
*
* - "0": The mouse is not captured while mouse buttons are pressed.
* - "1": The mouse is captured while mouse buttons are pressed.
*
* By default the mouse is captured while mouse buttons are pressed so if the
* mouse is dragged outside the window, the application continues to receive
* mouse events until the button is released.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_MOUSE_AUTO_CAPTURE: Hint = Hint { name: "SDL_HINT_MOUSE_AUTO_CAPTURE", value: "SDL_MOUSE_AUTO_CAPTURE", doc: "/**\n* A variable controlling whether the mouse is captured while mouse buttons\n* are pressed.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": The mouse is not captured while mouse buttons are pressed.\n* - \"1\": The mouse is captured while mouse buttons are pressed.\n*\n* By default the mouse is captured while mouse buttons are pressed so if the\n* mouse is dragged outside the window, the application continues to receive\n* mouse events until the button is released.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable setting the double click radius, in pixels.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_MOUSE_DOUBLE_CLICK_RADIUS: Hint = Hint { name: "SDL_HINT_MOUSE_DOUBLE_CLICK_RADIUS", value: "SDL_MOUSE_DOUBLE_CLICK_RADIUS", doc: "/**\n* A variable setting the double click radius, in pixels.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable setting the double click time, in milliseconds.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_MOUSE_DOUBLE_CLICK_TIME: Hint = Hint { name: "SDL_HINT_MOUSE_DOUBLE_CLICK_TIME", value: "SDL_MOUSE_DOUBLE_CLICK_TIME", doc: "/**\n* A variable setting the double click time, in milliseconds.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable setting which system cursor to use as the default cursor.
*
* This should be an integer corresponding to the SDL_SystemCursor enum. The
* default value is zero (SDL_SYSTEM_CURSOR_DEFAULT).
*
* This hint needs to be set before SDL_Init().
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_MOUSE_DEFAULT_SYSTEM_CURSOR: Hint = Hint { name: "SDL_HINT_MOUSE_DEFAULT_SYSTEM_CURSOR", value: "SDL_MOUSE_DEFAULT_SYSTEM_CURSOR", doc: "/**\n* A variable setting which system cursor to use as the default cursor.\n*\n* This should be an integer corresponding to the SDL_SystemCursor enum. The\n* default value is zero (SDL_SYSTEM_CURSOR_DEFAULT).\n*\n* This hint needs to be set before SDL_Init().\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable setting whether we should scale cursors by the current display
* scale.
*
* The variable can be set to the following values:
*
* - "0": Cursors will not change size based on the display content scale.
*   (default)
* - "1": Cursors will automatically match the display content scale (e.g. a
*   2x sized cursor will be used when the window is on a monitor with 200%
*   scale). This is currently implemented on Windows.
*
* This hint needs to be set before creating cursors.
*
* \since This hint is available since SDL 3.4.0.
*/
pub const SDL_HINT_MOUSE_DPI_SCALE_CURSORS: Hint = Hint { name: "SDL_HINT_MOUSE_DPI_SCALE_CURSORS", value: "SDL_MOUSE_DPI_SCALE_CURSORS", doc: "/**\n* A variable setting whether we should scale cursors by the current display\n* scale.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Cursors will not change size based on the display content scale.\n*   (default)\n* - \"1\": Cursors will automatically match the display content scale (e.g. a\n*   2x sized cursor will be used when the window is on a monitor with 200%\n*   scale). This is currently implemented on Windows.\n*\n* This hint needs to be set before creating cursors.\n*\n* \\since This hint is available since SDL 3.4.0.\n*/\n" };
/**
* A variable controlling whether warping a hidden mouse cursor will activate
* relative mouse mode.
*
* When this hint is set, the mouse cursor is hidden, and multiple warps to
* the window center occur within a short time period, SDL will emulate mouse
* warps using relative mouse mode. This can provide smoother and more
* reliable mouse motion for some older games, which continuously calculate
* the distance traveled by the mouse pointer and warp it back to the center
* of the window, rather than using relative mouse motion.
*
* Note that relative mouse mode may have different mouse acceleration
* behavior than pointer warps.
*
* If your application needs to repeatedly warp the hidden mouse cursor at a
* high-frequency for other purposes, it should disable this hint.
*
* The variable can be set to the following values:
*
* - "0": Attempts to warp the mouse will always be made.
* - "1": Some mouse warps will be emulated by forcing relative mouse mode.
*   (default)
*
* If not set, this is automatically enabled unless an application uses
* relative mouse mode directly.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_MOUSE_EMULATE_WARP_WITH_RELATIVE: Hint = Hint { name: "SDL_HINT_MOUSE_EMULATE_WARP_WITH_RELATIVE", value: "SDL_MOUSE_EMULATE_WARP_WITH_RELATIVE", doc: "/**\n* A variable controlling whether warping a hidden mouse cursor will activate\n* relative mouse mode.\n*\n* When this hint is set, the mouse cursor is hidden, and multiple warps to\n* the window center occur within a short time period, SDL will emulate mouse\n* warps using relative mouse mode. This can provide smoother and more\n* reliable mouse motion for some older games, which continuously calculate\n* the distance traveled by the mouse pointer and warp it back to the center\n* of the window, rather than using relative mouse motion.\n*\n* Note that relative mouse mode may have different mouse acceleration\n* behavior than pointer warps.\n*\n* If your application needs to repeatedly warp the hidden mouse cursor at a\n* high-frequency for other purposes, it should disable this hint.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Attempts to warp the mouse will always be made.\n* - \"1\": Some mouse warps will be emulated by forcing relative mouse mode.\n*   (default)\n*\n* If not set, this is automatically enabled unless an application uses\n* relative mouse mode directly.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* Allow mouse click events when clicking to focus an SDL window.
*
* The variable can be set to the following values:
*
* - "0": Ignore mouse clicks that activate a window. (default)
* - "1": Generate events for mouse clicks that activate a window.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_MOUSE_FOCUS_CLICKTHROUGH: Hint = Hint { name: "SDL_HINT_MOUSE_FOCUS_CLICKTHROUGH", value: "SDL_MOUSE_FOCUS_CLICKTHROUGH", doc: "/**\n* Allow mouse click events when clicking to focus an SDL window.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Ignore mouse clicks that activate a window. (default)\n* - \"1\": Generate events for mouse clicks that activate a window.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable setting the speed scale for mouse motion, in floating point,
* when the mouse is not in relative mode.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_MOUSE_NORMAL_SPEED_SCALE: Hint = Hint { name: "SDL_HINT_MOUSE_NORMAL_SPEED_SCALE", value: "SDL_MOUSE_NORMAL_SPEED_SCALE", doc: "/**\n* A variable setting the speed scale for mouse motion, in floating point,\n* when the mouse is not in relative mode.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether relative mouse mode constrains the mouse to
* the center of the window.
*
* Constraining to the center of the window works better for FPS games and
* when the application is running over RDP. Constraining to the whole window
* works better for 2D games and increases the chance that the mouse will be
* in the correct position when using high DPI mice.
*
* The variable can be set to the following values:
*
* - "0": Relative mouse mode constrains the mouse to the window.
* - "1": Relative mouse mode constrains the mouse to the center of the
*   window. (default)
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_MOUSE_RELATIVE_MODE_CENTER: Hint = Hint { name: "SDL_HINT_MOUSE_RELATIVE_MODE_CENTER", value: "SDL_MOUSE_RELATIVE_MODE_CENTER", doc: "/**\n* A variable controlling whether relative mouse mode constrains the mouse to\n* the center of the window.\n*\n* Constraining to the center of the window works better for FPS games and\n* when the application is running over RDP. Constraining to the whole window\n* works better for 2D games and increases the chance that the mouse will be\n* in the correct position when using high DPI mice.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Relative mouse mode constrains the mouse to the window.\n* - \"1\": Relative mouse mode constrains the mouse to the center of the\n*   window. (default)\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable setting the scale for mouse motion, in floating point, when the
* mouse is in relative mode.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_MOUSE_RELATIVE_SPEED_SCALE: Hint = Hint { name: "SDL_HINT_MOUSE_RELATIVE_SPEED_SCALE", value: "SDL_MOUSE_RELATIVE_SPEED_SCALE", doc: "/**\n* A variable setting the scale for mouse motion, in floating point, when the\n* mouse is in relative mode.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the system mouse acceleration curve is used
* for relative mouse motion.
*
* The variable can be set to the following values:
*
* - "0": Relative mouse motion will be unscaled. (default)
* - "1": Relative mouse motion will be scaled using the system mouse
*   acceleration curve.
*
* If SDL_HINT_MOUSE_RELATIVE_SPEED_SCALE is set, that will be applied after
* system speed scale.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_MOUSE_RELATIVE_SYSTEM_SCALE: Hint = Hint { name: "SDL_HINT_MOUSE_RELATIVE_SYSTEM_SCALE", value: "SDL_MOUSE_RELATIVE_SYSTEM_SCALE", doc: "/**\n* A variable controlling whether the system mouse acceleration curve is used\n* for relative mouse motion.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Relative mouse motion will be unscaled. (default)\n* - \"1\": Relative mouse motion will be scaled using the system mouse\n*   acceleration curve.\n*\n* If SDL_HINT_MOUSE_RELATIVE_SPEED_SCALE is set, that will be applied after\n* system speed scale.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether a motion event should be generated for mouse
* warping in relative mode.
*
* The variable can be set to the following values:
*
* - "0": Warping the mouse will not generate a motion event in relative mode
* - "1": Warping the mouse will generate a motion event in relative mode
*
* By default warping the mouse will not generate motion events in relative
* mode. This avoids the application having to filter out large relative
* motion due to warping.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_MOUSE_RELATIVE_WARP_MOTION: Hint = Hint { name: "SDL_HINT_MOUSE_RELATIVE_WARP_MOTION", value: "SDL_MOUSE_RELATIVE_WARP_MOTION", doc: "/**\n* A variable controlling whether a motion event should be generated for mouse\n* warping in relative mode.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Warping the mouse will not generate a motion event in relative mode\n* - \"1\": Warping the mouse will generate a motion event in relative mode\n*\n* By default warping the mouse will not generate motion events in relative\n* mode. This avoids the application having to filter out large relative\n* motion due to warping.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the hardware cursor stays visible when
* relative mode is active.
*
* This variable can be set to the following values:
*
* - "0": The cursor will be hidden while relative mode is active (default)
* - "1": The cursor will remain visible while relative mode is active
*
* Note that for systems without raw hardware inputs, relative mode is
* implemented using warping, so the hardware cursor will visibly warp between
* frames if this is enabled on those systems.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_MOUSE_RELATIVE_CURSOR_VISIBLE: Hint = Hint { name: "SDL_HINT_MOUSE_RELATIVE_CURSOR_VISIBLE", value: "SDL_MOUSE_RELATIVE_CURSOR_VISIBLE", doc: "/**\n* A variable controlling whether the hardware cursor stays visible when\n* relative mode is active.\n*\n* This variable can be set to the following values:\n*\n* - \"0\": The cursor will be hidden while relative mode is active (default)\n* - \"1\": The cursor will remain visible while relative mode is active\n*\n* Note that for systems without raw hardware inputs, relative mode is\n* implemented using warping, so the hardware cursor will visibly warp between\n* frames if this is enabled on those systems.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether mouse events should generate synthetic touch
* events.
*
* The variable can be set to the following values:
*
* - "0": Mouse events will not generate touch events. (default for desktop
*   platforms)
* - "1": Mouse events will generate touch events. (default for mobile
*   platforms, such as Android and iOS)
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_MOUSE_TOUCH_EVENTS: Hint = Hint { name: "SDL_HINT_MOUSE_TOUCH_EVENTS", value: "SDL_MOUSE_TOUCH_EVENTS", doc: "/**\n* A variable controlling whether mouse events should generate synthetic touch\n* events.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Mouse events will not generate touch events. (default for desktop\n*   platforms)\n* - \"1\": Mouse events will generate touch events. (default for mobile\n*   platforms, such as Android and iOS)\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the keyboard should be muted on the console.
*
* Normally the keyboard is muted while SDL applications are running so that
* keyboard input doesn't show up as key strokes on the console. This hint
* allows you to turn that off for debugging purposes.
*
* The variable can be set to the following values:
*
* - "0": Allow keystrokes to go through to the console.
* - "1": Mute keyboard input so it doesn't show up on the console. (default)
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_MUTE_CONSOLE_KEYBOARD: Hint = Hint { name: "SDL_HINT_MUTE_CONSOLE_KEYBOARD", value: "SDL_MUTE_CONSOLE_KEYBOARD", doc: "/**\n* A variable controlling whether the keyboard should be muted on the console.\n*\n* Normally the keyboard is muted while SDL applications are running so that\n* keyboard input doesn't show up as key strokes on the console. This hint\n* allows you to turn that off for debugging purposes.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Allow keystrokes to go through to the console.\n* - \"1\": Mute keyboard input so it doesn't show up on the console. (default)\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* Tell SDL not to catch the SIGINT or SIGTERM signals on POSIX platforms.
*
* The variable can be set to the following values:
*
* - "0": SDL will install a SIGINT and SIGTERM handler, and when it catches a
*   signal, convert it into an SDL_EVENT_QUIT event. (default)
* - "1": SDL will not install a signal handler at all.
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_NO_SIGNAL_HANDLERS: Hint = Hint { name: "SDL_HINT_NO_SIGNAL_HANDLERS", value: "SDL_NO_SIGNAL_HANDLERS", doc: "/**\n* Tell SDL not to catch the SIGINT or SIGTERM signals on POSIX platforms.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": SDL will install a SIGINT and SIGTERM handler, and when it catches a\n*   signal, convert it into an SDL_EVENT_QUIT event. (default)\n* - \"1\": SDL will not install a signal handler at all.\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* Specify the OpenGL library to load.
*
* This hint should be set before creating an OpenGL window or creating an
* OpenGL context. If this hint isn't set, SDL will choose a reasonable
* default.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_OPENGL_LIBRARY: Hint = Hint { name: "SDL_HINT_OPENGL_LIBRARY", value: "SDL_OPENGL_LIBRARY", doc: "/**\n* Specify the OpenGL library to load.\n*\n* This hint should be set before creating an OpenGL window or creating an\n* OpenGL context. If this hint isn't set, SDL will choose a reasonable\n* default.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* Specify the EGL library to load.
*
* This hint should be set before creating an OpenGL window or creating an
* OpenGL context. This hint is only considered if SDL is using EGL to manage
* OpenGL contexts. If this hint isn't set, SDL will choose a reasonable
* default.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_EGL_LIBRARY: Hint = Hint { name: "SDL_HINT_EGL_LIBRARY", value: "SDL_EGL_LIBRARY", doc: "/**\n* Specify the EGL library to load.\n*\n* This hint should be set before creating an OpenGL window or creating an\n* OpenGL context. This hint is only considered if SDL is using EGL to manage\n* OpenGL contexts. If this hint isn't set, SDL will choose a reasonable\n* default.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling what driver to use for OpenGL ES contexts.
*
* On some platforms, currently Windows and X11, OpenGL drivers may support
* creating contexts with an OpenGL ES profile. By default SDL uses these
* profiles, when available, otherwise it attempts to load an OpenGL ES
* library, e.g. that provided by the ANGLE project. This variable controls
* whether SDL follows this default behaviour or will always load an OpenGL ES
* library.
*
* Circumstances where this is useful include:
*
* - Testing an app with a particular OpenGL ES implementation, e.g ANGLE, or
*   emulator, e.g. those from ARM, Imagination or Qualcomm.
* - Resolving OpenGL ES function addresses at link time by linking with the
*   OpenGL ES library instead of querying them at run time with
*   SDL_GL_GetProcAddress().
*
* Caution: for an application to work with the default behaviour across
* different OpenGL drivers it must query the OpenGL ES function addresses at
* run time using SDL_GL_GetProcAddress().
*
* This variable is ignored on most platforms because OpenGL ES is native or
* not supported.
*
* The variable can be set to the following values:
*
* - "0": Use ES profile of OpenGL, if available. (default)
* - "1": Load OpenGL ES library using the default library names.
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_OPENGL_ES_DRIVER: Hint = Hint { name: "SDL_HINT_OPENGL_ES_DRIVER", value: "SDL_OPENGL_ES_DRIVER", doc: "/**\n* A variable controlling what driver to use for OpenGL ES contexts.\n*\n* On some platforms, currently Windows and X11, OpenGL drivers may support\n* creating contexts with an OpenGL ES profile. By default SDL uses these\n* profiles, when available, otherwise it attempts to load an OpenGL ES\n* library, e.g. that provided by the ANGLE project. This variable controls\n* whether SDL follows this default behaviour or will always load an OpenGL ES\n* library.\n*\n* Circumstances where this is useful include:\n*\n* - Testing an app with a particular OpenGL ES implementation, e.g ANGLE, or\n*   emulator, e.g. those from ARM, Imagination or Qualcomm.\n* - Resolving OpenGL ES function addresses at link time by linking with the\n*   OpenGL ES library instead of querying them at run time with\n*   SDL_GL_GetProcAddress().\n*\n* Caution: for an application to work with the default behaviour across\n* different OpenGL drivers it must query the OpenGL ES function addresses at\n* run time using SDL_GL_GetProcAddress().\n*\n* This variable is ignored on most platforms because OpenGL ES is native or\n* not supported.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Use ES profile of OpenGL, if available. (default)\n* - \"1\": Load OpenGL ES library using the default library names.\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether to force an sRGB-capable OpenGL context.
*
* At OpenGL context creation time, some platforms can request an sRGB-capable
* context. However, sometimes any form of the request can cause surprising
* results on some drivers, platforms, and hardware. Usually the surprise is
* in the form of rendering that is either a little darker or a little
* brighter than intended.
*
* This hint allows the user to override the app's sRGB requests and either
* force a specific value, or avoid requesting anything at all, depending on
* what makes things work correctly for their system.
*
* This is meant as a fail-safe; apps should probably not explicitly set this,
* and most users should not, either.
*
* Note that some platforms cannot make this request at all, and on all
* platforms this request can be denied by the operating system.
*
* In addition to attempting to obtain the type of sRGB-capable OpenGL context
* requested by this hint, SDL will try to force the state of
* GL_FRAMEBUFFER_SRGB on the new context, if appropriate.
*
* The variable can be set to the following values:
*
* - "0": Force a request for an OpenGL context that is _not_ sRGB-capable.
* - "1": Force a request for an OpenGL context that _is_ sRGB-capable.
* - "skip": Don't make any request for an sRGB-capable context (don't specify
*   the attribute at all during context creation time).
* - any other string is undefined behavior.
*
* If unset, or set to an empty string, SDL will make a request using the
* value the app specified with the SDL_GL_FRAMEBUFFER_SRGB_CAPABLE attribute.
*
* This hint should be set before an OpenGL context is created.
*
* \since This hint is available since SDL 3.4.2.
*/
pub const SDL_HINT_OPENGL_FORCE_SRGB_FRAMEBUFFER: Hint = Hint { name: "SDL_HINT_OPENGL_FORCE_SRGB_FRAMEBUFFER", value: "SDL_OPENGL_FORCE_SRGB_FRAMEBUFFER", doc: "/**\n* A variable controlling whether to force an sRGB-capable OpenGL context.\n*\n* At OpenGL context creation time, some platforms can request an sRGB-capable\n* context. However, sometimes any form of the request can cause surprising\n* results on some drivers, platforms, and hardware. Usually the surprise is\n* in the form of rendering that is either a little darker or a little\n* brighter than intended.\n*\n* This hint allows the user to override the app's sRGB requests and either\n* force a specific value, or avoid requesting anything at all, depending on\n* what makes things work correctly for their system.\n*\n* This is meant as a fail-safe; apps should probably not explicitly set this,\n* and most users should not, either.\n*\n* Note that some platforms cannot make this request at all, and on all\n* platforms this request can be denied by the operating system.\n*\n* In addition to attempting to obtain the type of sRGB-capable OpenGL context\n* requested by this hint, SDL will try to force the state of\n* GL_FRAMEBUFFER_SRGB on the new context, if appropriate.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Force a request for an OpenGL context that is _not_ sRGB-capable.\n* - \"1\": Force a request for an OpenGL context that _is_ sRGB-capable.\n* - \"skip\": Don't make any request for an sRGB-capable context (don't specify\n*   the attribute at all during context creation time).\n* - any other string is undefined behavior.\n*\n* If unset, or set to an empty string, SDL will make a request using the\n* value the app specified with the SDL_GL_FRAMEBUFFER_SRGB_CAPABLE attribute.\n*\n* This hint should be set before an OpenGL context is created.\n*\n* \\since This hint is available since SDL 3.4.2.\n*/\n" };
/**
* Mechanism to specify openvr_api library location
*
* By default, when using the OpenVR driver, it will search for the API
* library in the current folder. But, if you wish to use a system API you can
* specify that by using this hint. This should be the full or relative path
* to a .dll on Windows or .so on Linux.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_OPENVR_LIBRARY: Hint = Hint { name: "SDL_HINT_OPENVR_LIBRARY", value: "SDL_OPENVR_LIBRARY", doc: "/**\n* Mechanism to specify openvr_api library location\n*\n* By default, when using the OpenVR driver, it will search for the API\n* library in the current folder. But, if you wish to use a system API you can\n* specify that by using this hint. This should be the full or relative path\n* to a .dll on Windows or .so on Linux.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling which orientations are allowed on iOS/Android.
*
* In some circumstances it is necessary to be able to explicitly control
* which UI orientations are allowed.
*
* This variable is a space delimited list of the following values:
*
* - "LandscapeLeft"
* - "LandscapeRight"
* - "Portrait"
* - "PortraitUpsideDown"
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_ORIENTATIONS: Hint = Hint { name: "SDL_HINT_ORIENTATIONS", value: "SDL_ORIENTATIONS", doc: "/**\n* A variable controlling which orientations are allowed on iOS/Android.\n*\n* In some circumstances it is necessary to be able to explicitly control\n* which UI orientations are allowed.\n*\n* This variable is a space delimited list of the following values:\n*\n* - \"LandscapeLeft\"\n* - \"LandscapeRight\"\n* - \"Portrait\"\n* - \"PortraitUpsideDown\"\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling the use of a sentinel event when polling the event
* queue.
*
* When polling for events, SDL_PumpEvents is used to gather new events from
* devices. If a device keeps producing new events between calls to
* SDL_PumpEvents, a poll loop will become stuck until the new events stop.
* This is most noticeable when moving a high frequency mouse.
*
* The variable can be set to the following values:
*
* - "0": Disable poll sentinels.
* - "1": Enable poll sentinels. (default)
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_POLL_SENTINEL: Hint = Hint { name: "SDL_HINT_POLL_SENTINEL", value: "SDL_POLL_SENTINEL", doc: "/**\n* A variable controlling the use of a sentinel event when polling the event\n* queue.\n*\n* When polling for events, SDL_PumpEvents is used to gather new events from\n* devices. If a device keeps producing new events between calls to\n* SDL_PumpEvents, a poll loop will become stuck until the new events stop.\n* This is most noticeable when moving a high frequency mouse.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Disable poll sentinels.\n* - \"1\": Enable poll sentinels. (default)\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* Override for SDL_GetPreferredLocales().
*
* If set, this will be favored over anything the OS might report for the
* user's preferred locales. Changing this hint at runtime will not generate a
* SDL_EVENT_LOCALE_CHANGED event (but if you can change the hint, you can
* push your own event, if you want).
*
* The format of this hint is a comma-separated list of language and locale,
* combined with an underscore, as is a common format: "en_GB". Locale is
* optional: "en". So you might have a list like this: "en_GB,jp,es_PT"
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_PREFERRED_LOCALES: Hint = Hint { name: "SDL_HINT_PREFERRED_LOCALES", value: "SDL_PREFERRED_LOCALES", doc: "/**\n* Override for SDL_GetPreferredLocales().\n*\n* If set, this will be favored over anything the OS might report for the\n* user's preferred locales. Changing this hint at runtime will not generate a\n* SDL_EVENT_LOCALE_CHANGED event (but if you can change the hint, you can\n* push your own event, if you want).\n*\n* The format of this hint is a comma-separated list of language and locale,\n* combined with an underscore, as is a common format: \"en_GB\". Locale is\n* optional: \"en\". So you might have a list like this: \"en_GB,jp,es_PT\"\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable that decides whether to send SDL_EVENT_QUIT when closing the
* last window.
*
* The variable can be set to the following values:
*
* - "0": SDL will not send an SDL_EVENT_QUIT event when the last window is
*   requesting to close. Note that in this case, there are still other
*   legitimate reasons one might get an SDL_EVENT_QUIT event: choosing "Quit"
*   from the macOS menu bar, sending a SIGINT (ctrl-c) on Unix, etc.
* - "1": SDL will send a quit event when the last window is requesting to
*   close. (default)
*
* If there is at least one active system tray icon, SDL_EVENT_QUIT will
* instead be sent when both the last window will be closed and the last tray
* icon will be destroyed.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_QUIT_ON_LAST_WINDOW_CLOSE: Hint = Hint { name: "SDL_HINT_QUIT_ON_LAST_WINDOW_CLOSE", value: "SDL_QUIT_ON_LAST_WINDOW_CLOSE", doc: "/**\n* A variable that decides whether to send SDL_EVENT_QUIT when closing the\n* last window.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": SDL will not send an SDL_EVENT_QUIT event when the last window is\n*   requesting to close. Note that in this case, there are still other\n*   legitimate reasons one might get an SDL_EVENT_QUIT event: choosing \"Quit\"\n*   from the macOS menu bar, sending a SIGINT (ctrl-c) on Unix, etc.\n* - \"1\": SDL will send a quit event when the last window is requesting to\n*   close. (default)\n*\n* If there is at least one active system tray icon, SDL_EVENT_QUIT will\n* instead be sent when both the last window will be closed and the last tray\n* icon will be destroyed.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the Direct3D device is initialized for
* thread-safe operations.
*
* The variable can be set to the following values:
*
* - "0": Thread-safety is not enabled. (default)
* - "1": Thread-safety is enabled.
*
* This hint should be set before creating a renderer.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_RENDER_DIRECT3D_THREADSAFE: Hint = Hint { name: "SDL_HINT_RENDER_DIRECT3D_THREADSAFE", value: "SDL_RENDER_DIRECT3D_THREADSAFE", doc: "/**\n* A variable controlling whether the Direct3D device is initialized for\n* thread-safe operations.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Thread-safety is not enabled. (default)\n* - \"1\": Thread-safety is enabled.\n*\n* This hint should be set before creating a renderer.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether to enable Direct3D 11+'s Debug Layer.
*
* This variable does not have any effect on the Direct3D 9 based renderer.
*
* The variable can be set to the following values:
*
* - "0": Disable Debug Layer use. (default)
* - "1": Enable Debug Layer use.
*
* This hint should be set before creating a renderer.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_RENDER_DIRECT3D11_DEBUG: Hint = Hint { name: "SDL_HINT_RENDER_DIRECT3D11_DEBUG", value: "SDL_RENDER_DIRECT3D11_DEBUG", doc: "/**\n* A variable controlling whether to enable Direct3D 11+'s Debug Layer.\n*\n* This variable does not have any effect on the Direct3D 9 based renderer.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Disable Debug Layer use. (default)\n* - \"1\": Enable Debug Layer use.\n*\n* This hint should be set before creating a renderer.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether to use the Direct3D 11 WARP software
* rasterizer.
*
* For more information, see:
* https://learn.microsoft.com/en-us/windows/win32/direct3darticles/directx-warp
*
* The variable can be set to the following values:
*
* - "0": Disable WARP rasterizer. (default)
* - "1": Enable WARP rasterizer.
*
* This hint should be set before creating a renderer.
*
* \since This hint is available since SDL 3.4.0.
*/
pub const SDL_HINT_RENDER_DIRECT3D11_WARP: Hint = Hint { name: "SDL_HINT_RENDER_DIRECT3D11_WARP", value: "SDL_RENDER_DIRECT3D11_WARP", doc: "/**\n* A variable controlling whether to use the Direct3D 11 WARP software\n* rasterizer.\n*\n* For more information, see:\n* https://learn.microsoft.com/en-us/windows/win32/direct3darticles/directx-warp\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Disable WARP rasterizer. (default)\n* - \"1\": Enable WARP rasterizer.\n*\n* This hint should be set before creating a renderer.\n*\n* \\since This hint is available since SDL 3.4.0.\n*/\n" };
/**
* A variable controlling whether to enable Vulkan Validation Layers.
*
* This variable can be set to the following values:
*
* - "0": Disable Validation Layer use
* - "1": Enable Validation Layer use
*
* By default, SDL does not use Vulkan Validation Layers.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_RENDER_VULKAN_DEBUG: Hint = Hint { name: "SDL_HINT_RENDER_VULKAN_DEBUG", value: "SDL_RENDER_VULKAN_DEBUG", doc: "/**\n* A variable controlling whether to enable Vulkan Validation Layers.\n*\n* This variable can be set to the following values:\n*\n* - \"0\": Disable Validation Layer use\n* - \"1\": Enable Validation Layer use\n*\n* By default, SDL does not use Vulkan Validation Layers.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether to create the GPU device in debug mode.
*
* This variable can be set to the following values:
*
* - "0": Disable debug mode use (default)
* - "1": Enable debug mode use
*
* This hint should be set before creating a renderer.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_RENDER_GPU_DEBUG: Hint = Hint { name: "SDL_HINT_RENDER_GPU_DEBUG", value: "SDL_RENDER_GPU_DEBUG", doc: "/**\n* A variable controlling whether to create the GPU device in debug mode.\n*\n* This variable can be set to the following values:\n*\n* - \"0\": Disable debug mode use (default)\n* - \"1\": Enable debug mode use\n*\n* This hint should be set before creating a renderer.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether to prefer a low-power GPU on multi-GPU
* systems.
*
* This variable can be set to the following values:
*
* - "0": Prefer high-performance GPU (default)
* - "1": Prefer low-power GPU
*
* This hint should be set before creating a renderer.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_RENDER_GPU_LOW_POWER: Hint = Hint { name: "SDL_HINT_RENDER_GPU_LOW_POWER", value: "SDL_RENDER_GPU_LOW_POWER", doc: "/**\n* A variable controlling whether to prefer a low-power GPU on multi-GPU\n* systems.\n*\n* This variable can be set to the following values:\n*\n* - \"0\": Prefer high-performance GPU (default)\n* - \"1\": Prefer low-power GPU\n*\n* This hint should be set before creating a renderer.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable specifying which render driver to use.
*
* If the application doesn't pick a specific renderer to use, this variable
* specifies the name of the preferred renderer. If the preferred renderer
* can't be initialized, creating a renderer will fail.
*
* This variable is case insensitive and can be set to the following values:
*
* - "direct3d"
* - "direct3d11"
* - "direct3d12"
* - "opengl"
* - "opengles2"
* - "opengles"
* - "metal"
* - "vulkan"
* - "gpu"
* - "software"
*
* This hint accepts a comma-separated list of driver names, and each will be
* tried in the order listed when creating a renderer until one succeeds or
* all of them fail.
*
* The default varies by platform, but it's the first one in the list that is
* available on the current platform.
*
* This hint should be set before creating a renderer.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_RENDER_DRIVER: Hint = Hint { name: "SDL_HINT_RENDER_DRIVER", value: "SDL_RENDER_DRIVER", doc: "/**\n* A variable specifying which render driver to use.\n*\n* If the application doesn't pick a specific renderer to use, this variable\n* specifies the name of the preferred renderer. If the preferred renderer\n* can't be initialized, creating a renderer will fail.\n*\n* This variable is case insensitive and can be set to the following values:\n*\n* - \"direct3d\"\n* - \"direct3d11\"\n* - \"direct3d12\"\n* - \"opengl\"\n* - \"opengles2\"\n* - \"opengles\"\n* - \"metal\"\n* - \"vulkan\"\n* - \"gpu\"\n* - \"software\"\n*\n* This hint accepts a comma-separated list of driver names, and each will be\n* tried in the order listed when creating a renderer until one succeeds or\n* all of them fail.\n*\n* The default varies by platform, but it's the first one in the list that is\n* available on the current platform.\n*\n* This hint should be set before creating a renderer.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling how the 2D render API renders lines.
*
* The variable can be set to the following values:
*
* - "0": Use the default line drawing method (Bresenham's line algorithm)
* - "1": Use the driver point API using Bresenham's line algorithm (correct,
*   draws many points)
* - "2": Use the driver line API (occasionally misses line endpoints based on
*   hardware driver quirks
* - "3": Use the driver geometry API (correct, draws thicker diagonal lines)
*
* This hint should be set before creating a renderer.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_RENDER_LINE_METHOD: Hint = Hint { name: "SDL_HINT_RENDER_LINE_METHOD", value: "SDL_RENDER_LINE_METHOD", doc: "/**\n* A variable controlling how the 2D render API renders lines.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Use the default line drawing method (Bresenham's line algorithm)\n* - \"1\": Use the driver point API using Bresenham's line algorithm (correct,\n*   draws many points)\n* - \"2\": Use the driver line API (occasionally misses line endpoints based on\n*   hardware driver quirks\n* - \"3\": Use the driver geometry API (correct, draws thicker diagonal lines)\n*\n* This hint should be set before creating a renderer.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the Metal render driver select low power
* device over default one.
*
* The variable can be set to the following values:
*
* - "0": Use the preferred OS device. (default)
* - "1": Select a low power device.
*
* This hint should be set before creating a renderer.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_RENDER_METAL_PREFER_LOW_POWER_DEVICE: Hint = Hint { name: "SDL_HINT_RENDER_METAL_PREFER_LOW_POWER_DEVICE", value: "SDL_RENDER_METAL_PREFER_LOW_POWER_DEVICE", doc: "/**\n* A variable controlling whether the Metal render driver select low power\n* device over default one.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Use the preferred OS device. (default)\n* - \"1\": Select a low power device.\n*\n* This hint should be set before creating a renderer.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether updates to the SDL screen surface should be
* synchronized with the vertical refresh, to avoid tearing.
*
* This hint overrides the application preference when creating a renderer.
*
* The variable can be set to the following values:
*
* - "0": Disable vsync. (default)
* - "1": Enable vsync.
*
* This hint should be set before creating a renderer.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_RENDER_VSYNC: Hint = Hint { name: "SDL_HINT_RENDER_VSYNC", value: "SDL_RENDER_VSYNC", doc: "/**\n* A variable controlling whether updates to the SDL screen surface should be\n* synchronized with the vertical refresh, to avoid tearing.\n*\n* This hint overrides the application preference when creating a renderer.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Disable vsync. (default)\n* - \"1\": Enable vsync.\n*\n* This hint should be set before creating a renderer.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable to control whether the return key on the soft keyboard should
* hide the soft keyboard on Android and iOS.
*
* This hint sets the default value of SDL_PROP_TEXTINPUT_MULTILINE_BOOLEAN.
*
* The variable can be set to the following values:
*
* - "0": The return key will be handled as a key event. (default)
* - "1": The return key will hide the keyboard.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_RETURN_KEY_HIDES_IME: Hint = Hint { name: "SDL_HINT_RETURN_KEY_HIDES_IME", value: "SDL_RETURN_KEY_HIDES_IME", doc: "/**\n* A variable to control whether the return key on the soft keyboard should\n* hide the soft keyboard on Android and iOS.\n*\n* This hint sets the default value of SDL_PROP_TEXTINPUT_MULTILINE_BOOLEAN.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": The return key will be handled as a key event. (default)\n* - \"1\": The return key will hide the keyboard.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable containing a list of ROG gamepad capable mice.
*
* The format of the string is a comma separated list of USB VID/PID pairs in
* hexadecimal form, e.g.
*
* `0xAAAA/0xBBBB,0xCCCC/0xDDDD`
*
* The variable can also take the form of "@file", in which case the named
* file will be loaded and interpreted as the value of the variable.
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*
* \sa SDL_HINT_ROG_GAMEPAD_MICE_EXCLUDED
*/
pub const SDL_HINT_ROG_GAMEPAD_MICE: Hint = Hint { name: "SDL_HINT_ROG_GAMEPAD_MICE", value: "SDL_ROG_GAMEPAD_MICE", doc: "/**\n* A variable containing a list of ROG gamepad capable mice.\n*\n* The format of the string is a comma separated list of USB VID/PID pairs in\n* hexadecimal form, e.g.\n*\n* `0xAAAA/0xBBBB,0xCCCC/0xDDDD`\n*\n* The variable can also take the form of \"@file\", in which case the named\n* file will be loaded and interpreted as the value of the variable.\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*\n* \\sa SDL_HINT_ROG_GAMEPAD_MICE_EXCLUDED\n*/\n" };
/**
* A variable containing a list of devices that are not ROG gamepad capable
* mice.
*
* This will override SDL_HINT_ROG_GAMEPAD_MICE and the built in device list.
*
* The format of the string is a comma separated list of USB VID/PID pairs in
* hexadecimal form, e.g.
*
* `0xAAAA/0xBBBB,0xCCCC/0xDDDD`
*
* The variable can also take the form of "@file", in which case the named
* file will be loaded and interpreted as the value of the variable.
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_ROG_GAMEPAD_MICE_EXCLUDED: Hint = Hint { name: "SDL_HINT_ROG_GAMEPAD_MICE_EXCLUDED", value: "SDL_ROG_GAMEPAD_MICE_EXCLUDED", doc: "/**\n* A variable containing a list of devices that are not ROG gamepad capable\n* mice.\n*\n* This will override SDL_HINT_ROG_GAMEPAD_MICE and the built in device list.\n*\n* The format of the string is a comma separated list of USB VID/PID pairs in\n* hexadecimal form, e.g.\n*\n* `0xAAAA/0xBBBB,0xCCCC/0xDDDD`\n*\n* The variable can also take the form of \"@file\", in which case the named\n* file will be loaded and interpreted as the value of the variable.\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling the width of the PS2's framebuffer in pixels.
*
* By default, the variable is "640".
*
* \since This hint is available since SDL 3.4.0.
*/
pub const SDL_HINT_PS2_GS_WIDTH: Hint = Hint { name: "SDL_HINT_PS2_GS_WIDTH", value: "SDL_PS2_GS_WIDTH", doc: "/**\n* A variable controlling the width of the PS2's framebuffer in pixels.\n*\n* By default, the variable is \"640\".\n*\n* \\since This hint is available since SDL 3.4.0.\n*/\n" };
/**
* A variable controlling the height of the PS2's framebuffer in pixels.
*
* By default, the variable is "448".
*
* \since This hint is available since SDL 3.4.0.
*/
pub const SDL_HINT_PS2_GS_HEIGHT: Hint = Hint { name: "SDL_HINT_PS2_GS_HEIGHT", value: "SDL_PS2_GS_HEIGHT", doc: "/**\n* A variable controlling the height of the PS2's framebuffer in pixels.\n*\n* By default, the variable is \"448\".\n*\n* \\since This hint is available since SDL 3.4.0.\n*/\n" };
/**
* A variable controlling whether the signal is interlaced or progressive.
*
* The variable can be set to the following values:
*
* - "0": Image is interlaced. (default)
* - "1": Image is progressive.
*
* \since This hint is available since SDL 3.4.0.
*/
pub const SDL_HINT_PS2_GS_PROGRESSIVE: Hint = Hint { name: "SDL_HINT_PS2_GS_PROGRESSIVE", value: "SDL_PS2_GS_PROGRESSIVE", doc: "/**\n* A variable controlling whether the signal is interlaced or progressive.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Image is interlaced. (default)\n* - \"1\": Image is progressive.\n*\n* \\since This hint is available since SDL 3.4.0.\n*/\n" };
/**
* A variable controlling the video mode of the console.
*
* The variable can be set to the following values:
*
* - "": Console-native. (default)
* - "NTSC": 60hz region.
* - "PAL": 50hz region.
*
* \since This hint is available since SDL 3.4.0.
*/
pub const SDL_HINT_PS2_GS_MODE: Hint = Hint { name: "SDL_HINT_PS2_GS_MODE", value: "SDL_PS2_GS_MODE", doc: "/**\n* A variable controlling the video mode of the console.\n*\n* The variable can be set to the following values:\n*\n* - \"\": Console-native. (default)\n* - \"NTSC\": 60hz region.\n* - \"PAL\": 50hz region.\n*\n* \\since This hint is available since SDL 3.4.0.\n*/\n" };
/**
* A variable controlling which Dispmanx layer to use on a Raspberry PI.
*
* Also known as Z-order. The variable can take a negative or positive value.
* The default is 10000.
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_RPI_VIDEO_LAYER: Hint = Hint { name: "SDL_HINT_RPI_VIDEO_LAYER", value: "SDL_RPI_VIDEO_LAYER", doc: "/**\n* A variable controlling which Dispmanx layer to use on a Raspberry PI.\n*\n* Also known as Z-order. The variable can take a negative or positive value.\n* The default is 10000.\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* Specify an "activity name" for screensaver inhibition.
*
* Some platforms, notably Linux desktops, list the applications which are
* inhibiting the screensaver or other power-saving features.
*
* This hint lets you specify the "activity name" sent to the OS when
* SDL_DisableScreenSaver() is used (or the screensaver is automatically
* disabled). The contents of this hint are used when the screensaver is
* disabled. You should use a string that describes what your program is doing
* (and, therefore, why the screensaver is disabled). For example, "Playing a
* game" or "Watching a video".
*
* Setting this to "" or leaving it unset will have SDL use a reasonable
* default: "Playing a game" or something similar.
*
* This hint should be set before calling SDL_DisableScreenSaver()
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_SCREENSAVER_INHIBIT_ACTIVITY_NAME: Hint = Hint { name: "SDL_HINT_SCREENSAVER_INHIBIT_ACTIVITY_NAME", value: "SDL_SCREENSAVER_INHIBIT_ACTIVITY_NAME", doc: "/**\n* Specify an \"activity name\" for screensaver inhibition.\n*\n* Some platforms, notably Linux desktops, list the applications which are\n* inhibiting the screensaver or other power-saving features.\n*\n* This hint lets you specify the \"activity name\" sent to the OS when\n* SDL_DisableScreenSaver() is used (or the screensaver is automatically\n* disabled). The contents of this hint are used when the screensaver is\n* disabled. You should use a string that describes what your program is doing\n* (and, therefore, why the screensaver is disabled). For example, \"Playing a\n* game\" or \"Watching a video\".\n*\n* Setting this to \"\" or leaving it unset will have SDL use a reasonable\n* default: \"Playing a game\" or something similar.\n*\n* This hint should be set before calling SDL_DisableScreenSaver()\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether SDL calls dbus_shutdown() on quit.
*
* This is useful as a debug tool to validate memory leaks, but shouldn't ever
* be set in production applications, as other libraries used by the
* application might use dbus under the hood and this can cause crashes if
* they continue after SDL_Quit().
*
* The variable can be set to the following values:
*
* - "0": SDL will not call dbus_shutdown() on quit. (default)
* - "1": SDL will call dbus_shutdown() on quit.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_SHUTDOWN_DBUS_ON_QUIT: Hint = Hint { name: "SDL_HINT_SHUTDOWN_DBUS_ON_QUIT", value: "SDL_SHUTDOWN_DBUS_ON_QUIT", doc: "/**\n* A variable controlling whether SDL calls dbus_shutdown() on quit.\n*\n* This is useful as a debug tool to validate memory leaks, but shouldn't ever\n* be set in production applications, as other libraries used by the\n* application might use dbus under the hood and this can cause crashes if\n* they continue after SDL_Quit().\n*\n* The variable can be set to the following values:\n*\n* - \"0\": SDL will not call dbus_shutdown() on quit. (default)\n* - \"1\": SDL will call dbus_shutdown() on quit.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable that specifies a backend to use for title storage.
*
* By default, SDL will try all available storage backends in a reasonable
* order until it finds one that can work, but this hint allows the app or
* user to force a specific target, such as "pc" if, say, you are on Steam but
* want to avoid SteamRemoteStorage for title data.
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_STORAGE_TITLE_DRIVER: Hint = Hint { name: "SDL_HINT_STORAGE_TITLE_DRIVER", value: "SDL_STORAGE_TITLE_DRIVER", doc: "/**\n* A variable that specifies a backend to use for title storage.\n*\n* By default, SDL will try all available storage backends in a reasonable\n* order until it finds one that can work, but this hint allows the app or\n* user to force a specific target, such as \"pc\" if, say, you are on Steam but\n* want to avoid SteamRemoteStorage for title data.\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable that specifies a backend to use for user storage.
*
* By default, SDL will try all available storage backends in a reasonable
* order until it finds one that can work, but this hint allows the app or
* user to force a specific target, such as "pc" if, say, you are on Steam but
* want to avoid SteamRemoteStorage for user data.
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_STORAGE_USER_DRIVER: Hint = Hint { name: "SDL_HINT_STORAGE_USER_DRIVER", value: "SDL_STORAGE_USER_DRIVER", doc: "/**\n* A variable that specifies a backend to use for user storage.\n*\n* By default, SDL will try all available storage backends in a reasonable\n* order until it finds one that can work, but this hint allows the app or\n* user to force a specific target, such as \"pc\" if, say, you are on Steam but\n* want to avoid SteamRemoteStorage for user data.\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* Specifies whether SDL_THREAD_PRIORITY_TIME_CRITICAL should be treated as
* realtime.
*
* On some platforms, like Linux, a realtime priority thread may be subject to
* restrictions that require special handling by the application. This hint
* exists to let SDL know that the app is prepared to handle said
* restrictions.
*
* On Linux, SDL will apply the following configuration to any thread that
* becomes realtime:
*
* - The SCHED_RESET_ON_FORK bit will be set on the scheduling policy,
* - An RLIMIT_RTTIME budget will be configured to the rtkit specified limit.
* - Exceeding this limit will result in the kernel sending SIGKILL to the
*   app, refer to the man pages for more information.
*
* The variable can be set to the following values:
*
* - "0": default platform specific behaviour
* - "1": Force SDL_THREAD_PRIORITY_TIME_CRITICAL to a realtime scheduling
*   policy
*
* This hint should be set before calling SDL_SetCurrentThreadPriority()
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_THREAD_FORCE_REALTIME_TIME_CRITICAL: Hint = Hint { name: "SDL_HINT_THREAD_FORCE_REALTIME_TIME_CRITICAL", value: "SDL_THREAD_FORCE_REALTIME_TIME_CRITICAL", doc: "/**\n* Specifies whether SDL_THREAD_PRIORITY_TIME_CRITICAL should be treated as\n* realtime.\n*\n* On some platforms, like Linux, a realtime priority thread may be subject to\n* restrictions that require special handling by the application. This hint\n* exists to let SDL know that the app is prepared to handle said\n* restrictions.\n*\n* On Linux, SDL will apply the following configuration to any thread that\n* becomes realtime:\n*\n* - The SCHED_RESET_ON_FORK bit will be set on the scheduling policy,\n* - An RLIMIT_RTTIME budget will be configured to the rtkit specified limit.\n* - Exceeding this limit will result in the kernel sending SIGKILL to the\n*   app, refer to the man pages for more information.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": default platform specific behaviour\n* - \"1\": Force SDL_THREAD_PRIORITY_TIME_CRITICAL to a realtime scheduling\n*   policy\n*\n* This hint should be set before calling SDL_SetCurrentThreadPriority()\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A string specifying additional information to use with
* SDL_SetCurrentThreadPriority.
*
* By default SDL_SetCurrentThreadPriority will make appropriate system
* changes in order to apply a thread priority. For example on systems using
* pthreads the scheduler policy is changed automatically to a policy that
* works well with a given priority. Code which has specific requirements can
* override SDL's default behavior with this hint.
*
* pthread hint values are "current", "other", "fifo" and "rr". Currently no
* other platform hint values are defined but may be in the future.
*
* On Linux, the kernel may send SIGKILL to realtime tasks which exceed the
* distro configured execution budget for rtkit. This budget can be queried
* through RLIMIT_RTTIME after calling SDL_SetCurrentThreadPriority().
*
* This hint should be set before calling SDL_SetCurrentThreadPriority()
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_THREAD_PRIORITY_POLICY: Hint = Hint { name: "SDL_HINT_THREAD_PRIORITY_POLICY", value: "SDL_THREAD_PRIORITY_POLICY", doc: "/**\n* A string specifying additional information to use with\n* SDL_SetCurrentThreadPriority.\n*\n* By default SDL_SetCurrentThreadPriority will make appropriate system\n* changes in order to apply a thread priority. For example on systems using\n* pthreads the scheduler policy is changed automatically to a policy that\n* works well with a given priority. Code which has specific requirements can\n* override SDL's default behavior with this hint.\n*\n* pthread hint values are \"current\", \"other\", \"fifo\" and \"rr\". Currently no\n* other platform hint values are defined but may be in the future.\n*\n* On Linux, the kernel may send SIGKILL to realtime tasks which exceed the\n* distro configured execution budget for rtkit. This budget can be queried\n* through RLIMIT_RTTIME after calling SDL_SetCurrentThreadPriority().\n*\n* This hint should be set before calling SDL_SetCurrentThreadPriority()\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable that controls the timer resolution, in milliseconds.
*
* The higher resolution the timer, the more frequently the CPU services timer
* interrupts, and the more precise delays are, but this takes up power and
* CPU time. This hint is only used on Windows.
*
* See this blog post for more information:
* http://randomascii.wordpress.com/2013/07/08/windows-timer-resolution-megawatts-wasted/
*
* The default value is "1".
*
* If this variable is set to "0", the system timer resolution is not set.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_TIMER_RESOLUTION: Hint = Hint { name: "SDL_HINT_TIMER_RESOLUTION", value: "SDL_TIMER_RESOLUTION", doc: "/**\n* A variable that controls the timer resolution, in milliseconds.\n*\n* The higher resolution the timer, the more frequently the CPU services timer\n* interrupts, and the more precise delays are, but this takes up power and\n* CPU time. This hint is only used on Windows.\n*\n* See this blog post for more information:\n* http://randomascii.wordpress.com/2013/07/08/windows-timer-resolution-megawatts-wasted/\n*\n* The default value is \"1\".\n*\n* If this variable is set to \"0\", the system timer resolution is not set.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether touch events should generate synthetic mouse
* events.
*
* The variable can be set to the following values:
*
* - "0": Touch events will not generate mouse events.
* - "1": Touch events will generate mouse events. (default)
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_TOUCH_MOUSE_EVENTS: Hint = Hint { name: "SDL_HINT_TOUCH_MOUSE_EVENTS", value: "SDL_TOUCH_MOUSE_EVENTS", doc: "/**\n* A variable controlling whether touch events should generate synthetic mouse\n* events.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Touch events will not generate mouse events.\n* - \"1\": Touch events will generate mouse events. (default)\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether trackpads should be treated as touch
* devices.
*
* On macOS (and possibly other platforms in the future), SDL will report
* touches on a trackpad as mouse input, which is generally what users expect
* from this device; however, these are often actually full multitouch-capable
* touch devices, so it might be preferable to some apps to treat them as
* such.
*
* The variable can be set to the following values:
*
* - "0": Trackpad will send mouse events. (default)
* - "1": Trackpad will send touch events.
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_TRACKPAD_IS_TOUCH_ONLY: Hint = Hint { name: "SDL_HINT_TRACKPAD_IS_TOUCH_ONLY", value: "SDL_TRACKPAD_IS_TOUCH_ONLY", doc: "/**\n* A variable controlling whether trackpads should be treated as touch\n* devices.\n*\n* On macOS (and possibly other platforms in the future), SDL will report\n* touches on a trackpad as mouse input, which is generally what users expect\n* from this device; however, these are often actually full multitouch-capable\n* touch devices, so it might be preferable to some apps to treat them as\n* such.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Trackpad will send mouse events. (default)\n* - \"1\": Trackpad will send touch events.\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the Android / tvOS remotes should be listed
* as joystick devices, instead of sending keyboard events.
*
* The variable can be set to the following values:
*
* - "0": Remotes send enter/escape/arrow key events.
* - "1": Remotes are available as 2 axis, 2 button joysticks. (default)
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_TV_REMOTE_AS_JOYSTICK: Hint = Hint { name: "SDL_HINT_TV_REMOTE_AS_JOYSTICK", value: "SDL_TV_REMOTE_AS_JOYSTICK", doc: "/**\n* A variable controlling whether the Android / tvOS remotes should be listed\n* as joystick devices, instead of sending keyboard events.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Remotes send enter/escape/arrow key events.\n* - \"1\": Remotes are available as 2 axis, 2 button joysticks. (default)\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the screensaver is enabled.
*
* The variable can be set to the following values:
*
* - "0": Disable screensaver. (default)
* - "1": Enable screensaver.
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_VIDEO_ALLOW_SCREENSAVER: Hint = Hint { name: "SDL_HINT_VIDEO_ALLOW_SCREENSAVER", value: "SDL_VIDEO_ALLOW_SCREENSAVER", doc: "/**\n* A variable controlling whether the screensaver is enabled.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Disable screensaver. (default)\n* - \"1\": Enable screensaver.\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A comma separated list containing the names of the displays that SDL should
* sort to the front of the display list.
*
* When this hint is set, displays with matching name strings will be
* prioritized in the list of displays, as exposed by calling
* SDL_GetDisplays(), with the first listed becoming the primary display. The
* naming convention can vary depending on the environment, but it is usually
* a connector name (e.g. 'DP-1', 'DP-2', 'HDMI-A-1', etc...).
*
* On Wayland desktops, the connector names associated with displays can be
* found in the `name` property of the info output from `wayland-info -i
* wl_output`. On X11 desktops, the `xrandr` utility can be used to retrieve
* the connector names associated with displays.
*
* This hint is currently supported on the following drivers:
*
* - KMSDRM (kmsdrm)
* - Wayland (wayland)
* - X11 (x11)
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_VIDEO_DISPLAY_PRIORITY: Hint = Hint { name: "SDL_HINT_VIDEO_DISPLAY_PRIORITY", value: "SDL_VIDEO_DISPLAY_PRIORITY", doc: "/**\n* A comma separated list containing the names of the displays that SDL should\n* sort to the front of the display list.\n*\n* When this hint is set, displays with matching name strings will be\n* prioritized in the list of displays, as exposed by calling\n* SDL_GetDisplays(), with the first listed becoming the primary display. The\n* naming convention can vary depending on the environment, but it is usually\n* a connector name (e.g. 'DP-1', 'DP-2', 'HDMI-A-1', etc...).\n*\n* On Wayland desktops, the connector names associated with displays can be\n* found in the `name` property of the info output from `wayland-info -i\n* wl_output`. On X11 desktops, the `xrandr` utility can be used to retrieve\n* the connector names associated with displays.\n*\n* This hint is currently supported on the following drivers:\n*\n* - KMSDRM (kmsdrm)\n* - Wayland (wayland)\n* - X11 (x11)\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* Tell the video driver that we only want a double buffer.
*
* By default, most lowlevel 2D APIs will use a triple buffer scheme that
* wastes no CPU time on waiting for vsync after issuing a flip, but
* introduces a frame of latency. On the other hand, using a double buffer
* scheme instead is recommended for cases where low latency is an important
* factor because we save a whole frame of latency.
*
* We do so by waiting for vsync immediately after issuing a flip, usually
* just after eglSwapBuffers call in the backend's *_SwapWindow function.
*
* This hint is currently supported on the following drivers:
*
* - Raspberry Pi (raspberrypi)
* - Wayland (wayland)
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_VIDEO_DOUBLE_BUFFER: Hint = Hint { name: "SDL_HINT_VIDEO_DOUBLE_BUFFER", value: "SDL_VIDEO_DOUBLE_BUFFER", doc: "/**\n* Tell the video driver that we only want a double buffer.\n*\n* By default, most lowlevel 2D APIs will use a triple buffer scheme that\n* wastes no CPU time on waiting for vsync after issuing a flip, but\n* introduces a frame of latency. On the other hand, using a double buffer\n* scheme instead is recommended for cases where low latency is an important\n* factor because we save a whole frame of latency.\n*\n* We do so by waiting for vsync immediately after issuing a flip, usually\n* just after eglSwapBuffers call in the backend's *_SwapWindow function.\n*\n* This hint is currently supported on the following drivers:\n*\n* - Raspberry Pi (raspberrypi)\n* - Wayland (wayland)\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable that specifies a video backend to use.
*
* By default, SDL will try all available video backends in a reasonable order
* until it finds one that can work, but this hint allows the app or user to
* force a specific target, such as "x11" if, say, you are on Wayland but want
* to try talking to the X server instead.
*
* This hint accepts a comma-separated list of driver names, and each will be
* tried in the order listed during init, until one succeeds or all of them
* fail.
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_VIDEO_DRIVER: Hint = Hint { name: "SDL_HINT_VIDEO_DRIVER", value: "SDL_VIDEO_DRIVER", doc: "/**\n* A variable that specifies a video backend to use.\n*\n* By default, SDL will try all available video backends in a reasonable order\n* until it finds one that can work, but this hint allows the app or user to\n* force a specific target, such as \"x11\" if, say, you are on Wayland but want\n* to try talking to the X server instead.\n*\n* This hint accepts a comma-separated list of driver names, and each will be\n* tried in the order listed during init, until one succeeds or all of them\n* fail.\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the dummy video driver saves output frames.
*
* - "0": Video frames are not saved to disk. (default)
* - "1": Video frames are saved to files in the format "SDL_windowX-Y.bmp",
*   where X is the window ID, and Y is the frame number.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_VIDEO_DUMMY_SAVE_FRAMES: Hint = Hint { name: "SDL_HINT_VIDEO_DUMMY_SAVE_FRAMES", value: "SDL_VIDEO_DUMMY_SAVE_FRAMES", doc: "/**\n* A variable controlling whether the dummy video driver saves output frames.\n*\n* - \"0\": Video frames are not saved to disk. (default)\n* - \"1\": Video frames are saved to files in the format \"SDL_windowX-Y.bmp\",\n*   where X is the window ID, and Y is the frame number.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* If eglGetPlatformDisplay fails, fall back to calling eglGetDisplay.
*
* The variable can be set to one of the following values:
*
* - "0": Do not fall back to eglGetDisplay.
* - "1": Fall back to eglGetDisplay if eglGetPlatformDisplay fails. (default)
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_VIDEO_EGL_ALLOW_GETDISPLAY_FALLBACK: Hint = Hint { name: "SDL_HINT_VIDEO_EGL_ALLOW_GETDISPLAY_FALLBACK", value: "SDL_VIDEO_EGL_ALLOW_GETDISPLAY_FALLBACK", doc: "/**\n* If eglGetPlatformDisplay fails, fall back to calling eglGetDisplay.\n*\n* The variable can be set to one of the following values:\n*\n* - \"0\": Do not fall back to eglGetDisplay.\n* - \"1\": Fall back to eglGetDisplay if eglGetPlatformDisplay fails. (default)\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the OpenGL context should be created with
* EGL.
*
* The variable can be set to the following values:
*
* - "0": Use platform-specific GL context creation API (GLX, WGL, CGL, etc).
*   (default)
* - "1": Use EGL
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_VIDEO_FORCE_EGL: Hint = Hint { name: "SDL_HINT_VIDEO_FORCE_EGL", value: "SDL_VIDEO_FORCE_EGL", doc: "/**\n* A variable controlling whether the OpenGL context should be created with\n* EGL.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Use platform-specific GL context creation API (GLX, WGL, CGL, etc).\n*   (default)\n* - \"1\": Use EGL\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable that specifies the policy for fullscreen Spaces on macOS.
*
* The variable can be set to the following values:
*
* - "0": Disable Spaces support (FULLSCREEN_DESKTOP won't use them and
*   SDL_WINDOW_RESIZABLE windows won't offer the "fullscreen" button on their
*   titlebars).
* - "1": Enable Spaces support (FULLSCREEN_DESKTOP will use them and
*   SDL_WINDOW_RESIZABLE windows will offer the "fullscreen" button on their
*   titlebars). (default)
*
* This hint should be set before creating a window.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_VIDEO_MAC_FULLSCREEN_SPACES: Hint = Hint { name: "SDL_HINT_VIDEO_MAC_FULLSCREEN_SPACES", value: "SDL_VIDEO_MAC_FULLSCREEN_SPACES", doc: "/**\n* A variable that specifies the policy for fullscreen Spaces on macOS.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Disable Spaces support (FULLSCREEN_DESKTOP won't use them and\n*   SDL_WINDOW_RESIZABLE windows won't offer the \"fullscreen\" button on their\n*   titlebars).\n* - \"1\": Enable Spaces support (FULLSCREEN_DESKTOP will use them and\n*   SDL_WINDOW_RESIZABLE windows will offer the \"fullscreen\" button on their\n*   titlebars). (default)\n*\n* This hint should be set before creating a window.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable that specifies the menu visibility when a window is fullscreen
* in Spaces on macOS.
*
* The variable can be set to the following values:
*
* - "0": The menu will be hidden when the window is in a fullscreen space,
*   and not accessible by moving the mouse to the top of the screen.
* - "1": The menu will be accessible when the window is in a fullscreen
*   space.
* - "auto": The menu will be hidden if fullscreen mode was toggled on
*   programmatically via `SDL_SetWindowFullscreen()`, and accessible if
*   fullscreen was entered via the "fullscreen" button on the window title
*   bar. (default)
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_VIDEO_MAC_FULLSCREEN_MENU_VISIBILITY: Hint = Hint { name: "SDL_HINT_VIDEO_MAC_FULLSCREEN_MENU_VISIBILITY", value: "SDL_VIDEO_MAC_FULLSCREEN_MENU_VISIBILITY", doc: "/**\n* A variable that specifies the menu visibility when a window is fullscreen\n* in Spaces on macOS.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": The menu will be hidden when the window is in a fullscreen space,\n*   and not accessible by moving the mouse to the top of the screen.\n* - \"1\": The menu will be accessible when the window is in a fullscreen\n*   space.\n* - \"auto\": The menu will be hidden if fullscreen mode was toggled on\n*   programmatically via `SDL_SetWindowFullscreen()`, and accessible if\n*   fullscreen was entered via the \"fullscreen\" button on the window title\n*   bar. (default)\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable indicating whether the metal layer drawable size should be
* updated for the SDL_EVENT_WINDOW_PIXEL_SIZE_CHANGED event on macOS.
*
* The variable can be set to the following values:
*
* - "0": the metal layer drawable size will not be updated on the
*   SDL_EVENT_WINDOW_PIXEL_SIZE_CHANGED event.
* - "1": the metal layer drawable size will be updated on the
*   SDL_EVENT_WINDOW_PIXEL_SIZE_CHANGED event. (default)
*
* This hint should be set before SDL_Metal_CreateView called.
*
* \since This hint is available since SDL 3.4.0.
*/
pub const SDL_HINT_VIDEO_METAL_AUTO_RESIZE_DRAWABLE: Hint = Hint { name: "SDL_HINT_VIDEO_METAL_AUTO_RESIZE_DRAWABLE", value: "SDL_VIDEO_METAL_AUTO_RESIZE_DRAWABLE", doc: "/**\n* A variable indicating whether the metal layer drawable size should be\n* updated for the SDL_EVENT_WINDOW_PIXEL_SIZE_CHANGED event on macOS.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": the metal layer drawable size will not be updated on the\n*   SDL_EVENT_WINDOW_PIXEL_SIZE_CHANGED event.\n* - \"1\": the metal layer drawable size will be updated on the\n*   SDL_EVENT_WINDOW_PIXEL_SIZE_CHANGED event. (default)\n*\n* This hint should be set before SDL_Metal_CreateView called.\n*\n* \\since This hint is available since SDL 3.4.0.\n*/\n" };
/**
* A variable controlling whether SDL will attempt to automatically set the
* destination display to a mode most closely matching that of the previous
* display if an exclusive fullscreen window is moved onto it.
*
* The variable can be set to the following values:
*
* - "0": SDL will not attempt to automatically set a matching mode on the
*   destination display. If an exclusive fullscreen window is moved to a new
*   display, the window will become fullscreen desktop.
* - "1": SDL will attempt to automatically set a mode on the destination
*   display that most closely matches the mode of the display that the
*   exclusive fullscreen window was previously on. (default)
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.4.0.
*/
pub const SDL_HINT_VIDEO_MATCH_EXCLUSIVE_MODE_ON_MOVE: Hint = Hint { name: "SDL_HINT_VIDEO_MATCH_EXCLUSIVE_MODE_ON_MOVE", value: "SDL_VIDEO_MATCH_EXCLUSIVE_MODE_ON_MOVE", doc: "/**\n* A variable controlling whether SDL will attempt to automatically set the\n* destination display to a mode most closely matching that of the previous\n* display if an exclusive fullscreen window is moved onto it.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": SDL will not attempt to automatically set a matching mode on the\n*   destination display. If an exclusive fullscreen window is moved to a new\n*   display, the window will become fullscreen desktop.\n* - \"1\": SDL will attempt to automatically set a mode on the destination\n*   display that most closely matches the mode of the display that the\n*   exclusive fullscreen window was previously on. (default)\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.4.0.\n*/\n" };
/**
* A variable controlling whether fullscreen windows are minimized when they
* lose focus.
*
* The variable can be set to the following values:
*
* - "0": Fullscreen windows will not be minimized when they lose focus.
* - "1": Fullscreen windows are minimized when they lose focus.
* - "auto": Fullscreen windows are minimized when they lose focus if they use
*   exclusive fullscreen modes, so the desktop video mode is restored.
*   (default)
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_VIDEO_MINIMIZE_ON_FOCUS_LOSS: Hint = Hint { name: "SDL_HINT_VIDEO_MINIMIZE_ON_FOCUS_LOSS", value: "SDL_VIDEO_MINIMIZE_ON_FOCUS_LOSS", doc: "/**\n* A variable controlling whether fullscreen windows are minimized when they\n* lose focus.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Fullscreen windows will not be minimized when they lose focus.\n* - \"1\": Fullscreen windows are minimized when they lose focus.\n* - \"auto\": Fullscreen windows are minimized when they lose focus if they use\n*   exclusive fullscreen modes, so the desktop video mode is restored.\n*   (default)\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the offscreen video driver saves output
* frames.
*
* This only saves frames that are generated using software rendering, not
* accelerated OpenGL rendering.
*
* - "0": Video frames are not saved to disk. (default)
* - "1": Video frames are saved to files in the format "SDL_windowX-Y.bmp",
*   where X is the window ID, and Y is the frame number.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_VIDEO_OFFSCREEN_SAVE_FRAMES: Hint = Hint { name: "SDL_HINT_VIDEO_OFFSCREEN_SAVE_FRAMES", value: "SDL_VIDEO_OFFSCREEN_SAVE_FRAMES", doc: "/**\n* A variable controlling whether the offscreen video driver saves output\n* frames.\n*\n* This only saves frames that are generated using software rendering, not\n* accelerated OpenGL rendering.\n*\n* - \"0\": Video frames are not saved to disk. (default)\n* - \"1\": Video frames are saved to files in the format \"SDL_windowX-Y.bmp\",\n*   where X is the window ID, and Y is the frame number.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether all window operations will block until
* complete.
*
* Window systems that run asynchronously may not have the results of window
* operations that resize or move the window applied immediately upon the
* return of the requesting function. Setting this hint will cause such
* operations to block after every call until the pending operation has
* completed. Setting this to '1' is the equivalent of calling
* SDL_SyncWindow() after every function call.
*
* Be aware that amount of time spent blocking while waiting for window
* operations to complete can be quite lengthy, as animations may have to
* complete, which can take upwards of multiple seconds in some cases.
*
* The variable can be set to the following values:
*
* - "0": Window operations are non-blocking. (default)
* - "1": Window operations will block until completed.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_VIDEO_SYNC_WINDOW_OPERATIONS: Hint = Hint { name: "SDL_HINT_VIDEO_SYNC_WINDOW_OPERATIONS", value: "SDL_VIDEO_SYNC_WINDOW_OPERATIONS", doc: "/**\n* A variable controlling whether all window operations will block until\n* complete.\n*\n* Window systems that run asynchronously may not have the results of window\n* operations that resize or move the window applied immediately upon the\n* return of the requesting function. Setting this hint will cause such\n* operations to block after every call until the pending operation has\n* completed. Setting this to '1' is the equivalent of calling\n* SDL_SyncWindow() after every function call.\n*\n* Be aware that amount of time spent blocking while waiting for window\n* operations to complete can be quite lengthy, as animations may have to\n* complete, which can take upwards of multiple seconds in some cases.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Window operations are non-blocking. (default)\n* - \"1\": Window operations will block until completed.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the libdecor Wayland backend is allowed to
* be used.
*
* libdecor is used over xdg-shell when xdg-decoration protocol is
* unavailable.
*
* The variable can be set to the following values:
*
* - "0": libdecor use is disabled.
* - "1": libdecor use is enabled. (default)
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_VIDEO_WAYLAND_ALLOW_LIBDECOR: Hint = Hint { name: "SDL_HINT_VIDEO_WAYLAND_ALLOW_LIBDECOR", value: "SDL_VIDEO_WAYLAND_ALLOW_LIBDECOR", doc: "/**\n* A variable controlling whether the libdecor Wayland backend is allowed to\n* be used.\n*\n* libdecor is used over xdg-shell when xdg-decoration protocol is\n* unavailable.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": libdecor use is disabled.\n* - \"1\": libdecor use is enabled. (default)\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether video mode emulation is enabled under
* Wayland.
*
* When this hint is set, a standard set of emulated CVT video modes will be
* exposed for use by the application. If it is disabled, the only modes
* exposed will be the logical desktop size and, in the case of a scaled
* desktop, the native display resolution.
*
* The variable can be set to the following values:
*
* - "0": Video mode emulation is disabled.
* - "1": Video mode emulation is enabled. (default)
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_VIDEO_WAYLAND_MODE_EMULATION: Hint = Hint { name: "SDL_HINT_VIDEO_WAYLAND_MODE_EMULATION", value: "SDL_VIDEO_WAYLAND_MODE_EMULATION", doc: "/**\n* A variable controlling whether video mode emulation is enabled under\n* Wayland.\n*\n* When this hint is set, a standard set of emulated CVT video modes will be\n* exposed for use by the application. If it is disabled, the only modes\n* exposed will be the logical desktop size and, in the case of a scaled\n* desktop, the native display resolution.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Video mode emulation is disabled.\n* - \"1\": Video mode emulation is enabled. (default)\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling how modes with a non-native aspect ratio are
* displayed under Wayland.
*
* When this hint is set, the requested scaling will be used when displaying
* fullscreen video modes that don't match the display's native aspect ratio.
* This is contingent on compositor viewport support.
*
* The variable can be set to the following values:
*
* - "aspect" - Video modes will be displayed scaled, in their proper aspect
*   ratio, with black bars. (default)
* - "stretch" - Video modes will be scaled to fill the entire display.
* - "none" - Video modes will be displayed as 1:1 with no scaling.
*
* This hint should be set before creating a window.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_VIDEO_WAYLAND_MODE_SCALING: Hint = Hint { name: "SDL_HINT_VIDEO_WAYLAND_MODE_SCALING", value: "SDL_VIDEO_WAYLAND_MODE_SCALING", doc: "/**\n* A variable controlling how modes with a non-native aspect ratio are\n* displayed under Wayland.\n*\n* When this hint is set, the requested scaling will be used when displaying\n* fullscreen video modes that don't match the display's native aspect ratio.\n* This is contingent on compositor viewport support.\n*\n* The variable can be set to the following values:\n*\n* - \"aspect\" - Video modes will be displayed scaled, in their proper aspect\n*   ratio, with black bars. (default)\n* - \"stretch\" - Video modes will be scaled to fill the entire display.\n* - \"none\" - Video modes will be displayed as 1:1 with no scaling.\n*\n* This hint should be set before creating a window.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the libdecor Wayland backend is preferred
* over native decorations.
*
* When this hint is set, libdecor will be used to provide window decorations,
* even if xdg-decoration is available. (Note that, by default, libdecor will
* use xdg-decoration itself if available).
*
* The variable can be set to the following values:
*
* - "0": libdecor is enabled only if server-side decorations are unavailable.
*   (default)
* - "1": libdecor is always enabled if available.
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_VIDEO_WAYLAND_PREFER_LIBDECOR: Hint = Hint { name: "SDL_HINT_VIDEO_WAYLAND_PREFER_LIBDECOR", value: "SDL_VIDEO_WAYLAND_PREFER_LIBDECOR", doc: "/**\n* A variable controlling whether the libdecor Wayland backend is preferred\n* over native decorations.\n*\n* When this hint is set, libdecor will be used to provide window decorations,\n* even if xdg-decoration is available. (Note that, by default, libdecor will\n* use xdg-decoration itself if available).\n*\n* The variable can be set to the following values:\n*\n* - \"0\": libdecor is enabled only if server-side decorations are unavailable.\n*   (default)\n* - \"1\": libdecor is always enabled if available.\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable forcing non-DPI-aware Wayland windows to output at 1:1 scaling.
*
* This must be set before initializing the video subsystem.
*
* When this hint is set, Wayland windows that are not flagged as being
* DPI-aware will be output with scaling designed to force 1:1 pixel mapping.
*
* This is intended to allow legacy applications to be displayed without
* desktop scaling being applied, and has issues with certain display
* configurations, as this forces the window to behave in a way that Wayland
* desktops were not designed to accommodate:
*
* - Rounding errors can result with odd window sizes and/or desktop scales,
*   which can cause the window contents to appear slightly blurry.
* - Positioning the window may be imprecise due to unit conversions and
*   rounding.
* - The window may be unusably small on scaled desktops.
* - The window may jump in size when moving between displays of different
*   scale factors.
* - Displays may appear to overlap when using a multi-monitor setup with
*   scaling enabled.
* - Possible loss of cursor precision due to the logical size of the window
*   being reduced.
*
* New applications should be designed with proper DPI awareness handling
* instead of enabling this.
*
* The variable can be set to the following values:
*
* - "0": Windows will be scaled normally.
* - "1": Windows will be forced to scale to achieve 1:1 output.
*
* This hint should be set before creating a window.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_VIDEO_WAYLAND_SCALE_TO_DISPLAY: Hint = Hint { name: "SDL_HINT_VIDEO_WAYLAND_SCALE_TO_DISPLAY", value: "SDL_VIDEO_WAYLAND_SCALE_TO_DISPLAY", doc: "/**\n* A variable forcing non-DPI-aware Wayland windows to output at 1:1 scaling.\n*\n* This must be set before initializing the video subsystem.\n*\n* When this hint is set, Wayland windows that are not flagged as being\n* DPI-aware will be output with scaling designed to force 1:1 pixel mapping.\n*\n* This is intended to allow legacy applications to be displayed without\n* desktop scaling being applied, and has issues with certain display\n* configurations, as this forces the window to behave in a way that Wayland\n* desktops were not designed to accommodate:\n*\n* - Rounding errors can result with odd window sizes and/or desktop scales,\n*   which can cause the window contents to appear slightly blurry.\n* - Positioning the window may be imprecise due to unit conversions and\n*   rounding.\n* - The window may be unusably small on scaled desktops.\n* - The window may jump in size when moving between displays of different\n*   scale factors.\n* - Displays may appear to overlap when using a multi-monitor setup with\n*   scaling enabled.\n* - Possible loss of cursor precision due to the logical size of the window\n*   being reduced.\n*\n* New applications should be designed with proper DPI awareness handling\n* instead of enabling this.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Windows will be scaled normally.\n* - \"1\": Windows will be forced to scale to achieve 1:1 output.\n*\n* This hint should be set before creating a window.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable specifying which shader compiler to preload when using the
* Chrome ANGLE binaries.
*
* SDL has EGL and OpenGL ES2 support on Windows via the ANGLE project. It can
* use two different sets of binaries, those compiled by the user from source
* or those provided by the Chrome browser. In the later case, these binaries
* require that SDL loads a DLL providing the shader compiler.
*
* The variable can be set to the following values:
*
* - "d3dcompiler_46.dll" - best for Vista or later. (default)
* - "d3dcompiler_43.dll" - for XP support.
* - "none" - do not load any library, useful if you compiled ANGLE from
*   source and included the compiler in your binaries.
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_VIDEO_WIN_D3DCOMPILER: Hint = Hint { name: "SDL_HINT_VIDEO_WIN_D3DCOMPILER", value: "SDL_VIDEO_WIN_D3DCOMPILER", doc: "/**\n* A variable specifying which shader compiler to preload when using the\n* Chrome ANGLE binaries.\n*\n* SDL has EGL and OpenGL ES2 support on Windows via the ANGLE project. It can\n* use two different sets of binaries, those compiled by the user from source\n* or those provided by the Chrome browser. In the later case, these binaries\n* require that SDL loads a DLL providing the shader compiler.\n*\n* The variable can be set to the following values:\n*\n* - \"d3dcompiler_46.dll\" - best for Vista or later. (default)\n* - \"d3dcompiler_43.dll\" - for XP support.\n* - \"none\" - do not load any library, useful if you compiled ANGLE from\n*   source and included the compiler in your binaries.\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether SDL should call XSelectInput() to enable
* input events on X11 windows wrapped by SDL windows.
*
* The variable can be set to the following values:
*
* - "0": Don't call XSelectInput(), assuming the native window code has done
*   it already.
* - "1": Call XSelectInput() to enable input events. (default)
*
* This hint should be set before creating a window.
*
* \since This hint is available since SDL 3.2.10.
*/
pub const SDL_HINT_VIDEO_X11_EXTERNAL_WINDOW_INPUT: Hint = Hint { name: "SDL_HINT_VIDEO_X11_EXTERNAL_WINDOW_INPUT", value: "SDL_VIDEO_X11_EXTERNAL_WINDOW_INPUT", doc: "/**\n* A variable controlling whether SDL should call XSelectInput() to enable\n* input events on X11 windows wrapped by SDL windows.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Don't call XSelectInput(), assuming the native window code has done\n*   it already.\n* - \"1\": Call XSelectInput() to enable input events. (default)\n*\n* This hint should be set before creating a window.\n*\n* \\since This hint is available since SDL 3.2.10.\n*/\n" };
/**
* A variable controlling whether the X11 _NET_WM_BYPASS_COMPOSITOR hint
* should be used.
*
* The variable can be set to the following values:
*
* - "0": Disable _NET_WM_BYPASS_COMPOSITOR.
* - "1": Enable _NET_WM_BYPASS_COMPOSITOR. (default)
*
* This hint should be set before creating a window.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR: Hint = Hint { name: "SDL_HINT_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", value: "SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", doc: "/**\n* A variable controlling whether the X11 _NET_WM_BYPASS_COMPOSITOR hint\n* should be used.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Disable _NET_WM_BYPASS_COMPOSITOR.\n* - \"1\": Enable _NET_WM_BYPASS_COMPOSITOR. (default)\n*\n* This hint should be set before creating a window.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the X11 _NET_WM_PING protocol should be
* supported.
*
* By default SDL will use _NET_WM_PING, but for applications that know they
* will not always be able to respond to ping requests in a timely manner they
* can turn it off to avoid the window manager thinking the app is hung.
*
* The variable can be set to the following values:
*
* - "0": Disable _NET_WM_PING.
* - "1": Enable _NET_WM_PING. (default)
*
* This hint should be set before creating a window.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_VIDEO_X11_NET_WM_PING: Hint = Hint { name: "SDL_HINT_VIDEO_X11_NET_WM_PING", value: "SDL_VIDEO_X11_NET_WM_PING", doc: "/**\n* A variable controlling whether the X11 _NET_WM_PING protocol should be\n* supported.\n*\n* By default SDL will use _NET_WM_PING, but for applications that know they\n* will not always be able to respond to ping requests in a timely manner they\n* can turn it off to avoid the window manager thinking the app is hung.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Disable _NET_WM_PING.\n* - \"1\": Enable _NET_WM_PING. (default)\n*\n* This hint should be set before creating a window.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether SDL uses DirectColor visuals.
*
* The variable can be set to the following values:
*
* - "0": Disable DirectColor visuals.
* - "1": Enable DirectColor visuals. (default)
*
* This hint should be set before initializing the video subsystem.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_VIDEO_X11_NODIRECTCOLOR: Hint = Hint { name: "SDL_HINT_VIDEO_X11_NODIRECTCOLOR", value: "SDL_VIDEO_X11_NODIRECTCOLOR", doc: "/**\n* A variable controlling whether SDL uses DirectColor visuals.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Disable DirectColor visuals.\n* - \"1\": Enable DirectColor visuals. (default)\n*\n* This hint should be set before initializing the video subsystem.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable forcing the content scaling factor for X11 displays.
*
* The variable can be set to a floating point value in the range 1.0-10.0f
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_VIDEO_X11_SCALING_FACTOR: Hint = Hint { name: "SDL_HINT_VIDEO_X11_SCALING_FACTOR", value: "SDL_VIDEO_X11_SCALING_FACTOR", doc: "/**\n* A variable forcing the content scaling factor for X11 displays.\n*\n* The variable can be set to a floating point value in the range 1.0-10.0f\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable forcing the visual ID used for X11 display modes.
*
* This hint should be set before initializing the video subsystem.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_VIDEO_X11_VISUALID: Hint = Hint { name: "SDL_HINT_VIDEO_X11_VISUALID", value: "SDL_VIDEO_X11_VISUALID", doc: "/**\n* A variable forcing the visual ID used for X11 display modes.\n*\n* This hint should be set before initializing the video subsystem.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable forcing the visual ID chosen for new X11 windows.
*
* This hint should be set before creating a window.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_VIDEO_X11_WINDOW_VISUALID: Hint = Hint { name: "SDL_HINT_VIDEO_X11_WINDOW_VISUALID", value: "SDL_VIDEO_X11_WINDOW_VISUALID", doc: "/**\n* A variable forcing the visual ID chosen for new X11 windows.\n*\n* This hint should be set before creating a window.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the X11 XRandR extension should be used.
*
* The variable can be set to the following values:
*
* - "0": Disable XRandR.
* - "1": Enable XRandR. (default)
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_VIDEO_X11_XRANDR: Hint = Hint { name: "SDL_HINT_VIDEO_X11_XRANDR", value: "SDL_VIDEO_X11_XRANDR", doc: "/**\n* A variable controlling whether the X11 XRandR extension should be used.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Disable XRandR.\n* - \"1\": Enable XRandR. (default)\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether touch should be enabled on the back panel of
* the PlayStation Vita.
*
* The variable can be set to the following values:
*
* - "0": Disable touch on the back panel.
* - "1": Enable touch on the back panel. (default)
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_VITA_ENABLE_BACK_TOUCH: Hint = Hint { name: "SDL_HINT_VITA_ENABLE_BACK_TOUCH", value: "SDL_VITA_ENABLE_BACK_TOUCH", doc: "/**\n* A variable controlling whether touch should be enabled on the back panel of\n* the PlayStation Vita.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Disable touch on the back panel.\n* - \"1\": Enable touch on the back panel. (default)\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether touch should be enabled on the front panel
* of the PlayStation Vita.
*
* The variable can be set to the following values:
*
* - "0": Disable touch on the front panel.
* - "1": Enable touch on the front panel. (default)
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_VITA_ENABLE_FRONT_TOUCH: Hint = Hint { name: "SDL_HINT_VITA_ENABLE_FRONT_TOUCH", value: "SDL_VITA_ENABLE_FRONT_TOUCH", doc: "/**\n* A variable controlling whether touch should be enabled on the front panel\n* of the PlayStation Vita.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Disable touch on the front panel.\n* - \"1\": Enable touch on the front panel. (default)\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling the module path on the PlayStation Vita.
*
* This hint defaults to "app0:module"
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_VITA_MODULE_PATH: Hint = Hint { name: "SDL_HINT_VITA_MODULE_PATH", value: "SDL_VITA_MODULE_PATH", doc: "/**\n* A variable controlling the module path on the PlayStation Vita.\n*\n* This hint defaults to \"app0:module\"\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether to perform PVR initialization on the
* PlayStation Vita.
*
* - "0": Skip PVR initialization.
* - "1": Perform the normal PVR initialization. (default)
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_VITA_PVR_INIT: Hint = Hint { name: "SDL_HINT_VITA_PVR_INIT", value: "SDL_VITA_PVR_INIT", doc: "/**\n* A variable controlling whether to perform PVR initialization on the\n* PlayStation Vita.\n*\n* - \"0\": Skip PVR initialization.\n* - \"1\": Perform the normal PVR initialization. (default)\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable overriding the resolution reported on the PlayStation Vita.
*
* The variable can be set to the following values:
*
* - "544": 544p (default)
* - "720": 725p for PSTV
* - "1080": 1088i for PSTV
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_VITA_RESOLUTION: Hint = Hint { name: "SDL_HINT_VITA_RESOLUTION", value: "SDL_VITA_RESOLUTION", doc: "/**\n* A variable overriding the resolution reported on the PlayStation Vita.\n*\n* The variable can be set to the following values:\n*\n* - \"544\": 544p (default)\n* - \"720\": 725p for PSTV\n* - \"1080\": 1088i for PSTV\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether OpenGL should be used instead of OpenGL ES
* on the PlayStation Vita.
*
* The variable can be set to the following values:
*
* - "0": Use OpenGL ES. (default)
* - "1": Use OpenGL.
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_VITA_PVR_OPENGL: Hint = Hint { name: "SDL_HINT_VITA_PVR_OPENGL", value: "SDL_VITA_PVR_OPENGL", doc: "/**\n* A variable controlling whether OpenGL should be used instead of OpenGL ES\n* on the PlayStation Vita.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Use OpenGL ES. (default)\n* - \"1\": Use OpenGL.\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling which touchpad should generate synthetic mouse
* events.
*
* The variable can be set to the following values:
*
* - "0": Only front touchpad should generate mouse events. (default)
* - "1": Only back touchpad should generate mouse events.
* - "2": Both touchpads should generate mouse events.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_VITA_TOUCH_MOUSE_DEVICE: Hint = Hint { name: "SDL_HINT_VITA_TOUCH_MOUSE_DEVICE", value: "SDL_VITA_TOUCH_MOUSE_DEVICE", doc: "/**\n* A variable controlling which touchpad should generate synthetic mouse\n* events.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Only front touchpad should generate mouse events. (default)\n* - \"1\": Only back touchpad should generate mouse events.\n* - \"2\": Both touchpads should generate mouse events.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable overriding the display index used in SDL_Vulkan_CreateSurface()
*
* The display index starts at 0, which is the default.
*
* This hint should be set before calling SDL_Vulkan_CreateSurface()
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_VULKAN_DISPLAY: Hint = Hint { name: "SDL_HINT_VULKAN_DISPLAY", value: "SDL_VULKAN_DISPLAY", doc: "/**\n* A variable overriding the display index used in SDL_Vulkan_CreateSurface()\n*\n* The display index starts at 0, which is the default.\n*\n* This hint should be set before calling SDL_Vulkan_CreateSurface()\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* Specify the Vulkan library to load.
*
* This hint should be set before creating a Vulkan window or calling
* SDL_Vulkan_LoadLibrary().
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_VULKAN_LIBRARY: Hint = Hint { name: "SDL_HINT_VULKAN_LIBRARY", value: "SDL_VULKAN_LIBRARY", doc: "/**\n* Specify the Vulkan library to load.\n*\n* This hint should be set before creating a Vulkan window or calling\n* SDL_Vulkan_LoadLibrary().\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling how the fact chunk affects the loading of a WAVE
* file.
*
* The fact chunk stores information about the number of samples of a WAVE
* file. The Standards Update from Microsoft notes that this value can be used
* to 'determine the length of the data in seconds'. This is especially useful
* for compressed formats (for which this is a mandatory chunk) if they
* produce multiple sample frames per block and truncating the block is not
* allowed. The fact chunk can exactly specify how many sample frames there
* should be in this case.
*
* Unfortunately, most application seem to ignore the fact chunk and so SDL
* ignores it by default as well.
*
* The variable can be set to the following values:
*
* - "truncate" - Use the number of samples to truncate the wave data if the
*   fact chunk is present and valid.
* - "strict" - Like "truncate", but raise an error if the fact chunk is
*   invalid, not present for non-PCM formats, or if the data chunk doesn't
*   have that many samples.
* - "ignorezero" - Like "truncate", but ignore fact chunk if the number of
*   samples is zero.
* - "ignore" - Ignore fact chunk entirely. (default)
*
* This hint should be set before calling SDL_LoadWAV() or SDL_LoadWAV_IO()
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_WAVE_FACT_CHUNK: Hint = Hint { name: "SDL_HINT_WAVE_FACT_CHUNK", value: "SDL_WAVE_FACT_CHUNK", doc: "/**\n* A variable controlling how the fact chunk affects the loading of a WAVE\n* file.\n*\n* The fact chunk stores information about the number of samples of a WAVE\n* file. The Standards Update from Microsoft notes that this value can be used\n* to 'determine the length of the data in seconds'. This is especially useful\n* for compressed formats (for which this is a mandatory chunk) if they\n* produce multiple sample frames per block and truncating the block is not\n* allowed. The fact chunk can exactly specify how many sample frames there\n* should be in this case.\n*\n* Unfortunately, most application seem to ignore the fact chunk and so SDL\n* ignores it by default as well.\n*\n* The variable can be set to the following values:\n*\n* - \"truncate\" - Use the number of samples to truncate the wave data if the\n*   fact chunk is present and valid.\n* - \"strict\" - Like \"truncate\", but raise an error if the fact chunk is\n*   invalid, not present for non-PCM formats, or if the data chunk doesn't\n*   have that many samples.\n* - \"ignorezero\" - Like \"truncate\", but ignore fact chunk if the number of\n*   samples is zero.\n* - \"ignore\" - Ignore fact chunk entirely. (default)\n*\n* This hint should be set before calling SDL_LoadWAV() or SDL_LoadWAV_IO()\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling the maximum number of chunks in a WAVE file.
*
* This sets an upper bound on the number of chunks in a WAVE file to avoid
* wasting time on malformed or corrupt WAVE files. This defaults to "10000".
*
* This hint should be set before calling SDL_LoadWAV() or SDL_LoadWAV_IO()
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_WAVE_CHUNK_LIMIT: Hint = Hint { name: "SDL_HINT_WAVE_CHUNK_LIMIT", value: "SDL_WAVE_CHUNK_LIMIT", doc: "/**\n* A variable controlling the maximum number of chunks in a WAVE file.\n*\n* This sets an upper bound on the number of chunks in a WAVE file to avoid\n* wasting time on malformed or corrupt WAVE files. This defaults to \"10000\".\n*\n* This hint should be set before calling SDL_LoadWAV() or SDL_LoadWAV_IO()\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling how the size of the RIFF chunk affects the loading
* of a WAVE file.
*
* The size of the RIFF chunk (which includes all the sub-chunks of the WAVE
* file) is not always reliable. In case the size is wrong, it's possible to
* just ignore it and step through the chunks until a fixed limit is reached.
*
* Note that files that have trailing data unrelated to the WAVE file or
* corrupt files may slow down the loading process without a reliable
* boundary. By default, SDL stops after 10000 chunks to prevent wasting time.
* Use SDL_HINT_WAVE_CHUNK_LIMIT to adjust this value.
*
* The variable can be set to the following values:
*
* - "force" - Always use the RIFF chunk size as a boundary for the chunk
*   search.
* - "ignorezero" - Like "force", but a zero size searches up to 4 GiB.
*   (default)
* - "ignore" - Ignore the RIFF chunk size and always search up to 4 GiB.
* - "maximum" - Search for chunks until the end of file. (not recommended)
*
* This hint should be set before calling SDL_LoadWAV() or SDL_LoadWAV_IO()
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_WAVE_RIFF_CHUNK_SIZE: Hint = Hint { name: "SDL_HINT_WAVE_RIFF_CHUNK_SIZE", value: "SDL_WAVE_RIFF_CHUNK_SIZE", doc: "/**\n* A variable controlling how the size of the RIFF chunk affects the loading\n* of a WAVE file.\n*\n* The size of the RIFF chunk (which includes all the sub-chunks of the WAVE\n* file) is not always reliable. In case the size is wrong, it's possible to\n* just ignore it and step through the chunks until a fixed limit is reached.\n*\n* Note that files that have trailing data unrelated to the WAVE file or\n* corrupt files may slow down the loading process without a reliable\n* boundary. By default, SDL stops after 10000 chunks to prevent wasting time.\n* Use SDL_HINT_WAVE_CHUNK_LIMIT to adjust this value.\n*\n* The variable can be set to the following values:\n*\n* - \"force\" - Always use the RIFF chunk size as a boundary for the chunk\n*   search.\n* - \"ignorezero\" - Like \"force\", but a zero size searches up to 4 GiB.\n*   (default)\n* - \"ignore\" - Ignore the RIFF chunk size and always search up to 4 GiB.\n* - \"maximum\" - Search for chunks until the end of file. (not recommended)\n*\n* This hint should be set before calling SDL_LoadWAV() or SDL_LoadWAV_IO()\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling how a truncated WAVE file is handled.
*
* A WAVE file is considered truncated if any of the chunks are incomplete or
* the data chunk size is not a multiple of the block size. By default, SDL
* decodes until the first incomplete block, as most applications seem to do.
*
* The variable can be set to the following values:
*
* - "verystrict" - Raise an error if the file is truncated.
* - "strict" - Like "verystrict", but the size of the RIFF chunk is ignored.
* - "dropframe" - Decode until the first incomplete sample frame.
* - "dropblock" - Decode until the first incomplete block. (default)
*
* This hint should be set before calling SDL_LoadWAV() or SDL_LoadWAV_IO()
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_WAVE_TRUNCATION: Hint = Hint { name: "SDL_HINT_WAVE_TRUNCATION", value: "SDL_WAVE_TRUNCATION", doc: "/**\n* A variable controlling how a truncated WAVE file is handled.\n*\n* A WAVE file is considered truncated if any of the chunks are incomplete or\n* the data chunk size is not a multiple of the block size. By default, SDL\n* decodes until the first incomplete block, as most applications seem to do.\n*\n* The variable can be set to the following values:\n*\n* - \"verystrict\" - Raise an error if the file is truncated.\n* - \"strict\" - Like \"verystrict\", but the size of the RIFF chunk is ignored.\n* - \"dropframe\" - Decode until the first incomplete sample frame.\n* - \"dropblock\" - Decode until the first incomplete block. (default)\n*\n* This hint should be set before calling SDL_LoadWAV() or SDL_LoadWAV_IO()\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the window is activated when the
* SDL_RaiseWindow function is called.
*
* The variable can be set to the following values:
*
* - "0": The window is not activated when the SDL_RaiseWindow function is
*   called.
* - "1": The window is activated when the SDL_RaiseWindow function is called.
*   (default)
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_WINDOW_ACTIVATE_WHEN_RAISED: Hint = Hint { name: "SDL_HINT_WINDOW_ACTIVATE_WHEN_RAISED", value: "SDL_WINDOW_ACTIVATE_WHEN_RAISED", doc: "/**\n* A variable controlling whether the window is activated when the\n* SDL_RaiseWindow function is called.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": The window is not activated when the SDL_RaiseWindow function is\n*   called.\n* - \"1\": The window is activated when the SDL_RaiseWindow function is called.\n*   (default)\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the window is activated when the
* SDL_ShowWindow function is called.
*
* The variable can be set to the following values:
*
* - "0": The window is not activated when the SDL_ShowWindow function is
*   called.
* - "1": The window is activated when the SDL_ShowWindow function is called.
*   (default)
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_WINDOW_ACTIVATE_WHEN_SHOWN: Hint = Hint { name: "SDL_HINT_WINDOW_ACTIVATE_WHEN_SHOWN", value: "SDL_WINDOW_ACTIVATE_WHEN_SHOWN", doc: "/**\n* A variable controlling whether the window is activated when the\n* SDL_ShowWindow function is called.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": The window is not activated when the SDL_ShowWindow function is\n*   called.\n* - \"1\": The window is activated when the SDL_ShowWindow function is called.\n*   (default)\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* If set to "0" then never set the top-most flag on an SDL Window even if the
* application requests it.
*
* This is a debugging aid for developers and not expected to be used by end
* users.
*
* The variable can be set to the following values:
*
* - "0": don't allow topmost
* - "1": allow topmost (default)
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_WINDOW_ALLOW_TOPMOST: Hint = Hint { name: "SDL_HINT_WINDOW_ALLOW_TOPMOST", value: "SDL_WINDOW_ALLOW_TOPMOST", doc: "/**\n* If set to \"0\" then never set the top-most flag on an SDL Window even if the\n* application requests it.\n*\n* This is a debugging aid for developers and not expected to be used by end\n* users.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": don't allow topmost\n* - \"1\": allow topmost (default)\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the window frame and title bar are
* interactive when the cursor is hidden.
*
* The variable can be set to the following values:
*
* - "0": The window frame is not interactive when the cursor is hidden (no
*   move, resize, etc).
* - "1": The window frame is interactive when the cursor is hidden. (default)
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_WINDOW_FRAME_USABLE_WHILE_CURSOR_HIDDEN: Hint = Hint { name: "SDL_HINT_WINDOW_FRAME_USABLE_WHILE_CURSOR_HIDDEN", value: "SDL_WINDOW_FRAME_USABLE_WHILE_CURSOR_HIDDEN", doc: "/**\n* A variable controlling whether the window frame and title bar are\n* interactive when the cursor is hidden.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": The window frame is not interactive when the cursor is hidden (no\n*   move, resize, etc).\n* - \"1\": The window frame is interactive when the cursor is hidden. (default)\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether SDL generates window-close events for Alt+F4
* on Windows.
*
* The variable can be set to the following values:
*
* - "0": SDL will only do normal key handling for Alt+F4.
* - "1": SDL will generate a window-close event when it sees Alt+F4.
*   (default)
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_WINDOWS_CLOSE_ON_ALT_F4: Hint = Hint { name: "SDL_HINT_WINDOWS_CLOSE_ON_ALT_F4", value: "SDL_WINDOWS_CLOSE_ON_ALT_F4", doc: "/**\n* A variable controlling whether SDL generates window-close events for Alt+F4\n* on Windows.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": SDL will only do normal key handling for Alt+F4.\n* - \"1\": SDL will generate a window-close event when it sees Alt+F4.\n*   (default)\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether menus can be opened with their keyboard
* shortcut (Alt+mnemonic).
*
* If the mnemonics are enabled, then menus can be opened by pressing the Alt
* key and the corresponding mnemonic (for example, Alt+F opens the File
* menu). However, in case an invalid mnemonic is pressed, Windows makes an
* audible beep to convey that nothing happened. This is true even if the
* window has no menu at all!
*
* Because most SDL applications don't have menus, and some want to use the
* Alt key for other purposes, SDL disables mnemonics (and the beeping) by
* default.
*
* Note: This also affects keyboard events: with mnemonics enabled, when a
* menu is opened from the keyboard, you will not receive a KEYUP event for
* the mnemonic key, and *might* not receive one for Alt.
*
* The variable can be set to the following values:
*
* - "0": Alt+mnemonic does nothing, no beeping. (default)
* - "1": Alt+mnemonic opens menus, invalid mnemonics produce a beep.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_WINDOWS_ENABLE_MENU_MNEMONICS: Hint = Hint { name: "SDL_HINT_WINDOWS_ENABLE_MENU_MNEMONICS", value: "SDL_WINDOWS_ENABLE_MENU_MNEMONICS", doc: "/**\n* A variable controlling whether menus can be opened with their keyboard\n* shortcut (Alt+mnemonic).\n*\n* If the mnemonics are enabled, then menus can be opened by pressing the Alt\n* key and the corresponding mnemonic (for example, Alt+F opens the File\n* menu). However, in case an invalid mnemonic is pressed, Windows makes an\n* audible beep to convey that nothing happened. This is true even if the\n* window has no menu at all!\n*\n* Because most SDL applications don't have menus, and some want to use the\n* Alt key for other purposes, SDL disables mnemonics (and the beeping) by\n* default.\n*\n* Note: This also affects keyboard events: with mnemonics enabled, when a\n* menu is opened from the keyboard, you will not receive a KEYUP event for\n* the mnemonic key, and *might* not receive one for Alt.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Alt+mnemonic does nothing, no beeping. (default)\n* - \"1\": Alt+mnemonic opens menus, invalid mnemonics produce a beep.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether the windows message loop is processed by
* SDL.
*
* The variable can be set to the following values:
*
* - "0": The window message loop is not run.
* - "1": The window message loop is processed in SDL_PumpEvents(). (default)
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_WINDOWS_ENABLE_MESSAGELOOP: Hint = Hint { name: "SDL_HINT_WINDOWS_ENABLE_MESSAGELOOP", value: "SDL_WINDOWS_ENABLE_MESSAGELOOP", doc: "/**\n* A variable controlling whether the windows message loop is processed by\n* SDL.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": The window message loop is not run.\n* - \"1\": The window message loop is processed in SDL_PumpEvents(). (default)\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether GameInput is used for raw keyboard and mouse
* on Windows.
*
* The variable can be set to the following values:
*
* - "0": GameInput is not used for raw keyboard and mouse events. (default)
* - "1": GameInput is used for raw keyboard and mouse events, if available.
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_WINDOWS_GAMEINPUT: Hint = Hint { name: "SDL_HINT_WINDOWS_GAMEINPUT", value: "SDL_WINDOWS_GAMEINPUT", doc: "/**\n* A variable controlling whether GameInput is used for raw keyboard and mouse\n* on Windows.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": GameInput is not used for raw keyboard and mouse events. (default)\n* - \"1\": GameInput is used for raw keyboard and mouse events, if available.\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether raw keyboard events are used on Windows.
*
* The variable can be set to the following values:
*
* - "0": The Windows message loop is used for keyboard events. (default)
* - "1": Low latency raw keyboard events are used.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_WINDOWS_RAW_KEYBOARD: Hint = Hint { name: "SDL_HINT_WINDOWS_RAW_KEYBOARD", value: "SDL_WINDOWS_RAW_KEYBOARD", doc: "/**\n* A variable controlling whether raw keyboard events are used on Windows.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": The Windows message loop is used for keyboard events. (default)\n* - \"1\": Low latency raw keyboard events are used.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether or not the RIDEV_NOHOTKEYS flag is set when
* enabling Windows raw keyboard events.
*
* This blocks any hotkeys that have been registered by applications from
* having any effect beyond generating raw WM_INPUT events.
*
* This flag does not affect system-hotkeys like ALT-TAB or CTRL-ALT-DEL, but
* does affect the Windows Logo key since it is a userland hotkey registered
* by explorer.exe.
*
* The variable can be set to the following values:
*
* - "0": Hotkeys are not excluded. (default)
* - "1": Hotkeys are excluded.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.4.0.
*/
pub const SDL_HINT_WINDOWS_RAW_KEYBOARD_EXCLUDE_HOTKEYS: Hint = Hint { name: "SDL_HINT_WINDOWS_RAW_KEYBOARD_EXCLUDE_HOTKEYS", value: "SDL_WINDOWS_RAW_KEYBOARD_EXCLUDE_HOTKEYS", doc: "/**\n* A variable controlling whether or not the RIDEV_NOHOTKEYS flag is set when\n* enabling Windows raw keyboard events.\n*\n* This blocks any hotkeys that have been registered by applications from\n* having any effect beyond generating raw WM_INPUT events.\n*\n* This flag does not affect system-hotkeys like ALT-TAB or CTRL-ALT-DEL, but\n* does affect the Windows Logo key since it is a userland hotkey registered\n* by explorer.exe.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Hotkeys are not excluded. (default)\n* - \"1\": Hotkeys are excluded.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.4.0.\n*/\n" };
/**
* A variable controlling whether the RIDEV_INPUTSINK flag is set when
* enabling Windows raw keyboard events.
*
* This enables the window to still receive input even if not in foreground.
*
* Focused windows that receive text input will still prevent input events
* from triggering.
*
* - "0": Input is not received when not in focus or foreground. (default)
* - "1": Input will be received even when not in focus or foreground.
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.4.4.
*/
pub const SDL_HINT_WINDOWS_RAW_KEYBOARD_INPUTSINK: Hint = Hint { name: "SDL_HINT_WINDOWS_RAW_KEYBOARD_INPUTSINK", value: "SDL_WINDOWS_RAW_KEYBOARD_INPUTSINK", doc: "/**\n* A variable controlling whether the RIDEV_INPUTSINK flag is set when\n* enabling Windows raw keyboard events.\n*\n* This enables the window to still receive input even if not in foreground.\n*\n* Focused windows that receive text input will still prevent input events\n* from triggering.\n*\n* - \"0\": Input is not received when not in focus or foreground. (default)\n* - \"1\": Input will be received even when not in focus or foreground.\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.4.4.\n*/\n" };
/**
* A variable controlling whether SDL uses Kernel Semaphores on Windows.
*
* Kernel Semaphores are inter-process and require a context switch on every
* interaction. On Windows 8 and newer, the WaitOnAddress API is available.
* Using that and atomics to implement semaphores increases performance. SDL
* will fall back to Kernel Objects on older OS versions or if forced to by
* this hint.
*
* The variable can be set to the following values:
*
* - "0": Use Atomics and WaitOnAddress API when available, otherwise fall
*   back to Kernel Objects. (default)
* - "1": Force the use of Kernel Objects in all cases.
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_WINDOWS_FORCE_SEMAPHORE_KERNEL: Hint = Hint { name: "SDL_HINT_WINDOWS_FORCE_SEMAPHORE_KERNEL", value: "SDL_WINDOWS_FORCE_SEMAPHORE_KERNEL", doc: "/**\n* A variable controlling whether SDL uses Kernel Semaphores on Windows.\n*\n* Kernel Semaphores are inter-process and require a context switch on every\n* interaction. On Windows 8 and newer, the WaitOnAddress API is available.\n* Using that and atomics to implement semaphores increases performance. SDL\n* will fall back to Kernel Objects on older OS versions or if forced to by\n* this hint.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Use Atomics and WaitOnAddress API when available, otherwise fall\n*   back to Kernel Objects. (default)\n* - \"1\": Force the use of Kernel Objects in all cases.\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable to specify custom icon resource id from RC file on Windows
* platform.
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_WINDOWS_INTRESOURCE_ICON: Hint = Hint { name: "SDL_HINT_WINDOWS_INTRESOURCE_ICON", value: "SDL_WINDOWS_INTRESOURCE_ICON", doc: "/**\n* A variable to specify custom icon resource id from RC file on Windows\n* platform.\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable to specify custom icon resource id from RC file on Windows
* platform.
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_WINDOWS_INTRESOURCE_ICON_SMALL: Hint = Hint { name: "SDL_HINT_WINDOWS_INTRESOURCE_ICON_SMALL", value: "SDL_WINDOWS_INTRESOURCE_ICON_SMALL", doc: "/**\n* A variable to specify custom icon resource id from RC file on Windows\n* platform.\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether SDL uses the D3D9Ex API introduced in
* Windows Vista, instead of normal D3D9.
*
* Direct3D 9Ex contains changes to state management that can eliminate device
* loss errors during scenarios like Alt+Tab or UAC prompts. D3D9Ex may
* require some changes to your application to cope with the new behavior, so
* this is disabled by default.
*
* For more information on Direct3D 9Ex, see:
*
* - https://docs.microsoft.com/en-us/windows/win32/direct3darticles/graphics-apis-in-windows-vista#direct3d-9ex
* - https://docs.microsoft.com/en-us/windows/win32/direct3darticles/direct3d-9ex-improvements
*
* The variable can be set to the following values:
*
* - "0": Use the original Direct3D 9 API. (default)
* - "1": Use the Direct3D 9Ex API on Vista and later (and fall back if D3D9Ex
*   is unavailable)
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_WINDOWS_USE_D3D9EX: Hint = Hint { name: "SDL_HINT_WINDOWS_USE_D3D9EX", value: "SDL_WINDOWS_USE_D3D9EX", doc: "/**\n* A variable controlling whether SDL uses the D3D9Ex API introduced in\n* Windows Vista, instead of normal D3D9.\n*\n* Direct3D 9Ex contains changes to state management that can eliminate device\n* loss errors during scenarios like Alt+Tab or UAC prompts. D3D9Ex may\n* require some changes to your application to cope with the new behavior, so\n* this is disabled by default.\n*\n* For more information on Direct3D 9Ex, see:\n*\n* - https://docs.microsoft.com/en-us/windows/win32/direct3darticles/graphics-apis-in-windows-vista#direct3d-9ex\n* - https://docs.microsoft.com/en-us/windows/win32/direct3darticles/direct3d-9ex-improvements\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Use the original Direct3D 9 API. (default)\n* - \"1\": Use the Direct3D 9Ex API on Vista and later (and fall back if D3D9Ex\n*   is unavailable)\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether SDL will clear the window contents when the
* WM_ERASEBKGND message is received.
*
* The variable can be set to the following values:
*
* - "0"/"never": Never clear the window.
* - "1"/"initial": Clear the window when the first WM_ERASEBKGND event fires.
*   (default)
* - "2"/"always": Clear the window on every WM_ERASEBKGND event.
*
* This hint should be set before creating a window.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_WINDOWS_ERASE_BACKGROUND_MODE: Hint = Hint { name: "SDL_HINT_WINDOWS_ERASE_BACKGROUND_MODE", value: "SDL_WINDOWS_ERASE_BACKGROUND_MODE", doc: "/**\n* A variable controlling whether SDL will clear the window contents when the\n* WM_ERASEBKGND message is received.\n*\n* The variable can be set to the following values:\n*\n* - \"0\"/\"never\": Never clear the window.\n* - \"1\"/\"initial\": Clear the window when the first WM_ERASEBKGND event fires.\n*   (default)\n* - \"2\"/\"always\": Clear the window on every WM_ERASEBKGND event.\n*\n* This hint should be set before creating a window.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether X11 windows are marked as override-redirect.
*
* If set, this _might_ increase framerate at the expense of the desktop not
* working as expected. Override-redirect windows aren't noticed by the window
* manager at all.
*
* You should probably only use this for fullscreen windows, and you probably
* shouldn't even use it for that. But it's here if you want to try!
*
* The variable can be set to the following values:
*
* - "0": Do not mark the window as override-redirect. (default)
* - "1": Mark the window as override-redirect.
*
* This hint should be set before creating a window.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_X11_FORCE_OVERRIDE_REDIRECT: Hint = Hint { name: "SDL_HINT_X11_FORCE_OVERRIDE_REDIRECT", value: "SDL_X11_FORCE_OVERRIDE_REDIRECT", doc: "/**\n* A variable controlling whether X11 windows are marked as override-redirect.\n*\n* If set, this _might_ increase framerate at the expense of the desktop not\n* working as expected. Override-redirect windows aren't noticed by the window\n* manager at all.\n*\n* You should probably only use this for fullscreen windows, and you probably\n* shouldn't even use it for that. But it's here if you want to try!\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Do not mark the window as override-redirect. (default)\n* - \"1\": Mark the window as override-redirect.\n*\n* This hint should be set before creating a window.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable specifying the type of an X11 window.
*
* During SDL_CreateWindow, SDL uses the _NET_WM_WINDOW_TYPE X11 property to
* report to the window manager the type of window it wants to create. This
* might be set to various things if SDL_WINDOW_TOOLTIP or
* SDL_WINDOW_POPUP_MENU, etc, were specified. For "normal" windows that
* haven't set a specific type, this hint can be used to specify a custom
* type. For example, a dock window might set this to
* "_NET_WM_WINDOW_TYPE_DOCK".
*
* This hint should be set before creating a window.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_X11_WINDOW_TYPE: Hint = Hint { name: "SDL_HINT_X11_WINDOW_TYPE", value: "SDL_X11_WINDOW_TYPE", doc: "/**\n* A variable specifying the type of an X11 window.\n*\n* During SDL_CreateWindow, SDL uses the _NET_WM_WINDOW_TYPE X11 property to\n* report to the window manager the type of window it wants to create. This\n* might be set to various things if SDL_WINDOW_TOOLTIP or\n* SDL_WINDOW_POPUP_MENU, etc, were specified. For \"normal\" windows that\n* haven't set a specific type, this hint can be used to specify a custom\n* type. For example, a dock window might set this to\n* \"_NET_WM_WINDOW_TYPE_DOCK\".\n*\n* This hint should be set before creating a window.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* Specify the XCB library to load for the X11 driver.
*
* The default is platform-specific, often "libX11-xcb.so.1".
*
* This hint should be set before initializing the video subsystem.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_X11_XCB_LIBRARY: Hint = Hint { name: "SDL_HINT_X11_XCB_LIBRARY", value: "SDL_X11_XCB_LIBRARY", doc: "/**\n* Specify the XCB library to load for the X11 driver.\n*\n* The default is platform-specific, often \"libX11-xcb.so.1\".\n*\n* This hint should be set before initializing the video subsystem.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether XInput should be used for controller
* handling.
*
* The variable can be set to the following values:
*
* - "0": XInput is not enabled.
* - "1": XInput is enabled. (default)
*
* This hint should be set before SDL is initialized.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_XINPUT_ENABLED: Hint = Hint { name: "SDL_HINT_XINPUT_ENABLED", value: "SDL_XINPUT_ENABLED", doc: "/**\n* A variable controlling whether XInput should be used for controller\n* handling.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": XInput is not enabled.\n* - \"1\": XInput is enabled. (default)\n*\n* This hint should be set before SDL is initialized.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling response to SDL_assert failures.
*
* The variable can be set to the following case-sensitive values:
*
* - "abort": Program terminates immediately.
* - "break": Program triggers a debugger breakpoint.
* - "retry": Program reruns the SDL_assert's test again.
* - "ignore": Program continues on, ignoring this assertion failure this
*   time.
* - "always_ignore": Program continues on, ignoring this assertion failure
*   for the rest of the run.
*
* Note that SDL_SetAssertionHandler offers a programmatic means to deal with
* assertion failures through a callback, and this hint is largely intended to
* be used via environment variables by end users and automated tools.
*
* This hint should be set before an assertion failure is triggered and can be
* changed at any time.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_ASSERT: Hint = Hint { name: "SDL_HINT_ASSERT", value: "SDL_ASSERT", doc: "/**\n* A variable controlling response to SDL_assert failures.\n*\n* The variable can be set to the following case-sensitive values:\n*\n* - \"abort\": Program terminates immediately.\n* - \"break\": Program triggers a debugger breakpoint.\n* - \"retry\": Program reruns the SDL_assert's test again.\n* - \"ignore\": Program continues on, ignoring this assertion failure this\n*   time.\n* - \"always_ignore\": Program continues on, ignoring this assertion failure\n*   for the rest of the run.\n*\n* Note that SDL_SetAssertionHandler offers a programmatic means to deal with\n* assertion failures through a callback, and this hint is largely intended to\n* be used via environment variables by end users and automated tools.\n*\n* This hint should be set before an assertion failure is triggered and can be\n* changed at any time.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether pen events should generate synthetic mouse
* events.
*
* The variable can be set to the following values:
*
* - "0": Pen events will not generate mouse events.
* - "1": Pen events will generate mouse events. (default)
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_PEN_MOUSE_EVENTS: Hint = Hint { name: "SDL_HINT_PEN_MOUSE_EVENTS", value: "SDL_PEN_MOUSE_EVENTS", doc: "/**\n* A variable controlling whether pen events should generate synthetic mouse\n* events.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Pen events will not generate mouse events.\n* - \"1\": Pen events will generate mouse events. (default)\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
/**
* A variable controlling whether pen events should generate synthetic touch
* events.
*
* The variable can be set to the following values:
*
* - "0": Pen events will not generate touch events.
* - "1": Pen events will generate touch events. (default)
*
* This hint can be set anytime.
*
* \since This hint is available since SDL 3.2.0.
*/
pub const SDL_HINT_PEN_TOUCH_EVENTS: Hint = Hint { name: "SDL_HINT_PEN_TOUCH_EVENTS", value: "SDL_PEN_TOUCH_EVENTS", doc: "/**\n* A variable controlling whether pen events should generate synthetic touch\n* events.\n*\n* The variable can be set to the following values:\n*\n* - \"0\": Pen events will not generate touch events.\n* - \"1\": Pen events will generate touch events. (default)\n*\n* This hint can be set anytime.\n*\n* \\since This hint is available since SDL 3.2.0.\n*/\n" };
pub const SDL_PROP_AUDIOSTREAM_AUTO_CLEANUP_BOOLEAN: Property = Property { name: "SDL_PROP_AUDIOSTREAM_AUTO_CLEANUP_BOOLEAN", value: "SDL.audiostream.auto_cleanup", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_FILE_DIALOG_FILTERS_POINTER: Property = Property { name: "SDL_PROP_FILE_DIALOG_FILTERS_POINTER", value: "SDL.filedialog.filters", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_FILE_DIALOG_NFILTERS_NUMBER: Property = Property { name: "SDL_PROP_FILE_DIALOG_NFILTERS_NUMBER", value: "SDL.filedialog.nfilters", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_FILE_DIALOG_WINDOW_POINTER: Property = Property { name: "SDL_PROP_FILE_DIALOG_WINDOW_POINTER", value: "SDL.filedialog.window", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_FILE_DIALOG_LOCATION_STRING: Property = Property { name: "SDL_PROP_FILE_DIALOG_LOCATION_STRING", value: "SDL.filedialog.location", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_FILE_DIALOG_MANY_BOOLEAN: Property = Property { name: "SDL_PROP_FILE_DIALOG_MANY_BOOLEAN", value: "SDL.filedialog.many", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_FILE_DIALOG_TITLE_STRING: Property = Property { name: "SDL_PROP_FILE_DIALOG_TITLE_STRING", value: "SDL.filedialog.title", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_FILE_DIALOG_ACCEPT_STRING: Property = Property { name: "SDL_PROP_FILE_DIALOG_ACCEPT_STRING", value: "SDL.filedialog.accept", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_FILE_DIALOG_CANCEL_STRING: Property = Property { name: "SDL_PROP_FILE_DIALOG_CANCEL_STRING", value: "SDL.filedialog.cancel", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_GPU_DEVICE_CREATE_DEBUGMODE_BOOLEAN: Property = Property { name: "SDL_PROP_GPU_DEVICE_CREATE_DEBUGMODE_BOOLEAN", value: "SDL.gpu.device.create.debugmode", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_GPU_DEVICE_CREATE_PREFERLOWPOWER_BOOLEAN: Property = Property { name: "SDL_PROP_GPU_DEVICE_CREATE_PREFERLOWPOWER_BOOLEAN", value: "SDL.gpu.device.create.preferlowpower", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_GPU_DEVICE_CREATE_VERBOSE_BOOLEAN: Property = Property { name: "SDL_PROP_GPU_DEVICE_CREATE_VERBOSE_BOOLEAN", value: "SDL.gpu.device.create.verbose", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_GPU_DEVICE_CREATE_NAME_STRING: Property = Property { name: "SDL_PROP_GPU_DEVICE_CREATE_NAME_STRING", value: "SDL.gpu.device.create.name", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_GPU_DEVICE_CREATE_FEATURE_CLIP_DISTANCE_BOOLEAN: Property = Property { name: "SDL_PROP_GPU_DEVICE_CREATE_FEATURE_CLIP_DISTANCE_BOOLEAN", value: "SDL.gpu.device.create.feature.clip_distance", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_GPU_DEVICE_CREATE_FEATURE_DEPTH_CLAMPING_BOOLEAN: Property = Property { name: "SDL_PROP_GPU_DEVICE_CREATE_FEATURE_DEPTH_CLAMPING_BOOLEAN", value: "SDL.gpu.device.create.feature.depth_clamping", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_GPU_DEVICE_CREATE_FEATURE_INDIRECT_DRAW_FIRST_INSTANCE_BOOLEAN: Property = Property { name: "SDL_PROP_GPU_DEVICE_CREATE_FEATURE_INDIRECT_DRAW_FIRST_INSTANCE_BOOLEAN", value: "SDL.gpu.device.create.feature.indirect_draw_first_instance", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_GPU_DEVICE_CREATE_FEATURE_ANISOTROPY_BOOLEAN: Property = Property { name: "SDL_PROP_GPU_DEVICE_CREATE_FEATURE_ANISOTROPY_BOOLEAN", value: "SDL.gpu.device.create.feature.anisotropy", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_GPU_DEVICE_CREATE_SHADERS_PRIVATE_BOOLEAN: Property = Property { name: "SDL_PROP_GPU_DEVICE_CREATE_SHADERS_PRIVATE_BOOLEAN", value: "SDL.gpu.device.create.shaders.private", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_GPU_DEVICE_CREATE_SHADERS_SPIRV_BOOLEAN: Property = Property { name: "SDL_PROP_GPU_DEVICE_CREATE_SHADERS_SPIRV_BOOLEAN", value: "SDL.gpu.device.create.shaders.spirv", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_GPU_DEVICE_CREATE_SHADERS_DXBC_BOOLEAN: Property = Property { name: "SDL_PROP_GPU_DEVICE_CREATE_SHADERS_DXBC_BOOLEAN", value: "SDL.gpu.device.create.shaders.dxbc", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_GPU_DEVICE_CREATE_SHADERS_DXIL_BOOLEAN: Property = Property { name: "SDL_PROP_GPU_DEVICE_CREATE_SHADERS_DXIL_BOOLEAN", value: "SDL.gpu.device.create.shaders.dxil", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_GPU_DEVICE_CREATE_SHADERS_MSL_BOOLEAN: Property = Property { name: "SDL_PROP_GPU_DEVICE_CREATE_SHADERS_MSL_BOOLEAN", value: "SDL.gpu.device.create.shaders.msl", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_GPU_DEVICE_CREATE_SHADERS_METALLIB_BOOLEAN: Property = Property { name: "SDL_PROP_GPU_DEVICE_CREATE_SHADERS_METALLIB_BOOLEAN", value: "SDL.gpu.device.create.shaders.metallib", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_GPU_DEVICE_CREATE_D3D12_ALLOW_FEWER_RESOURCE_SLOTS_BOOLEAN: Property = Property { name: "SDL_PROP_GPU_DEVICE_CREATE_D3D12_ALLOW_FEWER_RESOURCE_SLOTS_BOOLEAN", value: "SDL.gpu.device.create.d3d12.allowtier1resourcebinding", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_GPU_DEVICE_CREATE_D3D12_SEMANTIC_NAME_STRING: Property = Property { name: "SDL_PROP_GPU_DEVICE_CREATE_D3D12_SEMANTIC_NAME_STRING", value: "SDL.gpu.device.create.d3d12.semantic", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_GPU_DEVICE_CREATE_D3D12_AGILITY_SDK_VERSION_NUMBER: Property = Property { name: "SDL_PROP_GPU_DEVICE_CREATE_D3D12_AGILITY_SDK_VERSION_NUMBER", value: "SDL.gpu.device.create.d3d12.agility_sdk_version", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_GPU_DEVICE_CREATE_D3D12_AGILITY_SDK_PATH_STRING: Property = Property { name: "SDL_PROP_GPU_DEVICE_CREATE_D3D12_AGILITY_SDK_PATH_STRING", value: "SDL.gpu.device.create.d3d12.agility_sdk_path", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_GPU_DEVICE_CREATE_VULKAN_REQUIRE_HARDWARE_ACCELERATION_BOOLEAN: Property = Property { name: "SDL_PROP_GPU_DEVICE_CREATE_VULKAN_REQUIRE_HARDWARE_ACCELERATION_BOOLEAN", value: "SDL.gpu.device.create.vulkan.requirehardwareacceleration", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_GPU_DEVICE_CREATE_VULKAN_OPTIONS_POINTER: Property = Property { name: "SDL_PROP_GPU_DEVICE_CREATE_VULKAN_OPTIONS_POINTER", value: "SDL.gpu.device.create.vulkan.options", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_GPU_DEVICE_CREATE_METAL_ALLOW_MACFAMILY1_BOOLEAN: Property = Property { name: "SDL_PROP_GPU_DEVICE_CREATE_METAL_ALLOW_MACFAMILY1_BOOLEAN", value: "SDL.gpu.device.create.metal.allowmacfamily1", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_GPU_DEVICE_CREATE_XR_ENABLE_BOOLEAN: Property = Property { name: "SDL_PROP_GPU_DEVICE_CREATE_XR_ENABLE_BOOLEAN", value: "SDL.gpu.device.create.xr.enable", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_GPU_DEVICE_CREATE_XR_INSTANCE_POINTER: Property = Property { name: "SDL_PROP_GPU_DEVICE_CREATE_XR_INSTANCE_POINTER", value: "SDL.gpu.device.create.xr.instance_out", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_GPU_DEVICE_CREATE_XR_SYSTEM_ID_POINTER: Property = Property { name: "SDL_PROP_GPU_DEVICE_CREATE_XR_SYSTEM_ID_POINTER", value: "SDL.gpu.device.create.xr.system_id_out", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_GPU_DEVICE_CREATE_XR_VERSION_NUMBER: Property = Property { name: "SDL_PROP_GPU_DEVICE_CREATE_XR_VERSION_NUMBER", value: "SDL.gpu.device.create.xr.version", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_GPU_DEVICE_CREATE_XR_FORM_FACTOR_NUMBER: Property = Property { name: "SDL_PROP_GPU_DEVICE_CREATE_XR_FORM_FACTOR_NUMBER", value: "SDL.gpu.device.create.xr.form_factor", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_GPU_DEVICE_CREATE_XR_EXTENSION_COUNT_NUMBER: Property = Property { name: "SDL_PROP_GPU_DEVICE_CREATE_XR_EXTENSION_COUNT_NUMBER", value: "SDL.gpu.device.create.xr.extensions.count", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_GPU_DEVICE_CREATE_XR_EXTENSION_NAMES_POINTER: Property = Property { name: "SDL_PROP_GPU_DEVICE_CREATE_XR_EXTENSION_NAMES_POINTER", value: "SDL.gpu.device.create.xr.extensions.names", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_GPU_DEVICE_CREATE_XR_LAYER_COUNT_NUMBER: Property = Property { name: "SDL_PROP_GPU_DEVICE_CREATE_XR_LAYER_COUNT_NUMBER", value: "SDL.gpu.device.create.xr.layers.count", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_GPU_DEVICE_CREATE_XR_LAYER_NAMES_POINTER: Property = Property { name: "SDL_PROP_GPU_DEVICE_CREATE_XR_LAYER_NAMES_POINTER", value: "SDL.gpu.device.create.xr.layers.names", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_GPU_DEVICE_CREATE_XR_APPLICATION_NAME_STRING: Property = Property { name: "SDL_PROP_GPU_DEVICE_CREATE_XR_APPLICATION_NAME_STRING", value: "SDL.gpu.device.create.xr.application.name", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_GPU_DEVICE_CREATE_XR_APPLICATION_VERSION_NUMBER: Property = Property { name: "SDL_PROP_GPU_DEVICE_CREATE_XR_APPLICATION_VERSION_NUMBER", value: "SDL.gpu.device.create.xr.application.version", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_GPU_DEVICE_CREATE_XR_ENGINE_NAME_STRING: Property = Property { name: "SDL_PROP_GPU_DEVICE_CREATE_XR_ENGINE_NAME_STRING", value: "SDL.gpu.device.create.xr.engine.name", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_GPU_DEVICE_CREATE_XR_ENGINE_VERSION_NUMBER: Property = Property { name: "SDL_PROP_GPU_DEVICE_CREATE_XR_ENGINE_VERSION_NUMBER", value: "SDL.gpu.device.create.xr.engine.version", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_GPU_DEVICE_NAME_STRING: Property = Property { name: "SDL_PROP_GPU_DEVICE_NAME_STRING", value: "SDL.gpu.device.name", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_GPU_DEVICE_DRIVER_NAME_STRING: Property = Property { name: "SDL_PROP_GPU_DEVICE_DRIVER_NAME_STRING", value: "SDL.gpu.device.driver_name", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_GPU_DEVICE_DRIVER_VERSION_STRING: Property = Property { name: "SDL_PROP_GPU_DEVICE_DRIVER_VERSION_STRING", value: "SDL.gpu.device.driver_version", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_GPU_DEVICE_DRIVER_INFO_STRING: Property = Property { name: "SDL_PROP_GPU_DEVICE_DRIVER_INFO_STRING", value: "SDL.gpu.device.driver_info", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_GPU_COMPUTEPIPELINE_CREATE_NAME_STRING: Property = Property { name: "SDL_PROP_GPU_COMPUTEPIPELINE_CREATE_NAME_STRING", value: "SDL.gpu.computepipeline.create.name", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_GPU_GRAPHICSPIPELINE_CREATE_NAME_STRING: Property = Property { name: "SDL_PROP_GPU_GRAPHICSPIPELINE_CREATE_NAME_STRING", value: "SDL.gpu.graphicspipeline.create.name", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_GPU_SAMPLER_CREATE_NAME_STRING: Property = Property { name: "SDL_PROP_GPU_SAMPLER_CREATE_NAME_STRING", value: "SDL.gpu.sampler.create.name", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_GPU_SHADER_CREATE_NAME_STRING: Property = Property { name: "SDL_PROP_GPU_SHADER_CREATE_NAME_STRING", value: "SDL.gpu.shader.create.name", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_R_FLOAT: Property = Property { name: "SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_R_FLOAT", value: "SDL.gpu.texture.create.d3d12.clear.r", ty: PropertyType::Float, doc: "" };
pub const SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_G_FLOAT: Property = Property { name: "SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_G_FLOAT", value: "SDL.gpu.texture.create.d3d12.clear.g", ty: PropertyType::Float, doc: "" };
pub const SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_B_FLOAT: Property = Property { name: "SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_B_FLOAT", value: "SDL.gpu.texture.create.d3d12.clear.b", ty: PropertyType::Float, doc: "" };
pub const SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_A_FLOAT: Property = Property { name: "SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_A_FLOAT", value: "SDL.gpu.texture.create.d3d12.clear.a", ty: PropertyType::Float, doc: "" };
pub const SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_DEPTH_FLOAT: Property = Property { name: "SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_DEPTH_FLOAT", value: "SDL.gpu.texture.create.d3d12.clear.depth", ty: PropertyType::Float, doc: "" };
pub const SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_STENCIL_NUMBER: Property = Property { name: "SDL_PROP_GPU_TEXTURE_CREATE_D3D12_CLEAR_STENCIL_NUMBER", value: "SDL.gpu.texture.create.d3d12.clear.stencil", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_GPU_TEXTURE_CREATE_NAME_STRING: Property = Property { name: "SDL_PROP_GPU_TEXTURE_CREATE_NAME_STRING", value: "SDL.gpu.texture.create.name", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_GPU_BUFFER_CREATE_NAME_STRING: Property = Property { name: "SDL_PROP_GPU_BUFFER_CREATE_NAME_STRING", value: "SDL.gpu.buffer.create.name", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_GPU_TRANSFERBUFFER_CREATE_NAME_STRING: Property = Property { name: "SDL_PROP_GPU_TRANSFERBUFFER_CREATE_NAME_STRING", value: "SDL.gpu.transferbuffer.create.name", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_HIDAPI_LIBUSB_DEVICE_HANDLE_POINTER: Property = Property { name: "SDL_PROP_HIDAPI_LIBUSB_DEVICE_HANDLE_POINTER", value: "SDL.hidapi.libusb.device.handle", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_APP_METADATA_NAME_STRING: Property = Property { name: "SDL_PROP_APP_METADATA_NAME_STRING", value: "SDL.app.metadata.name", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_APP_METADATA_VERSION_STRING: Property = Property { name: "SDL_PROP_APP_METADATA_VERSION_STRING", value: "SDL.app.metadata.version", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_APP_METADATA_IDENTIFIER_STRING: Property = Property { name: "SDL_PROP_APP_METADATA_IDENTIFIER_STRING", value: "SDL.app.metadata.identifier", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_APP_METADATA_CREATOR_STRING: Property = Property { name: "SDL_PROP_APP_METADATA_CREATOR_STRING", value: "SDL.app.metadata.creator", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_APP_METADATA_COPYRIGHT_STRING: Property = Property { name: "SDL_PROP_APP_METADATA_COPYRIGHT_STRING", value: "SDL.app.metadata.copyright", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_APP_METADATA_URL_STRING: Property = Property { name: "SDL_PROP_APP_METADATA_URL_STRING", value: "SDL.app.metadata.url", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_APP_METADATA_TYPE_STRING: Property = Property { name: "SDL_PROP_APP_METADATA_TYPE_STRING", value: "SDL.app.metadata.type", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_IOSTREAM_WINDOWS_HANDLE_POINTER: Property = Property { name: "SDL_PROP_IOSTREAM_WINDOWS_HANDLE_POINTER", value: "SDL.iostream.windows.handle", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_IOSTREAM_STDIO_FILE_POINTER: Property = Property { name: "SDL_PROP_IOSTREAM_STDIO_FILE_POINTER", value: "SDL.iostream.stdio.file", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_IOSTREAM_FILE_DESCRIPTOR_NUMBER: Property = Property { name: "SDL_PROP_IOSTREAM_FILE_DESCRIPTOR_NUMBER", value: "SDL.iostream.file_descriptor", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_IOSTREAM_ANDROID_AASSET_POINTER: Property = Property { name: "SDL_PROP_IOSTREAM_ANDROID_AASSET_POINTER", value: "SDL.iostream.android.aasset", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_IOSTREAM_MEMORY_POINTER: Property = Property { name: "SDL_PROP_IOSTREAM_MEMORY_POINTER", value: "SDL.iostream.memory.base", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_IOSTREAM_MEMORY_SIZE_NUMBER: Property = Property { name: "SDL_PROP_IOSTREAM_MEMORY_SIZE_NUMBER", value: "SDL.iostream.memory.size", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_IOSTREAM_MEMORY_FREE_FUNC_POINTER: Property = Property { name: "SDL_PROP_IOSTREAM_MEMORY_FREE_FUNC_POINTER", value: "SDL.iostream.memory.free", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_IOSTREAM_DYNAMIC_MEMORY_POINTER: Property = Property { name: "SDL_PROP_IOSTREAM_DYNAMIC_MEMORY_POINTER", value: "SDL.iostream.dynamic.memory", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_IOSTREAM_DYNAMIC_CHUNKSIZE_NUMBER: Property = Property { name: "SDL_PROP_IOSTREAM_DYNAMIC_CHUNKSIZE_NUMBER", value: "SDL.iostream.dynamic.chunksize", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_JOYSTICK_CAP_MONO_LED_BOOLEAN: Property = Property { name: "SDL_PROP_JOYSTICK_CAP_MONO_LED_BOOLEAN", value: "SDL.joystick.cap.mono_led", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_JOYSTICK_CAP_RGB_LED_BOOLEAN: Property = Property { name: "SDL_PROP_JOYSTICK_CAP_RGB_LED_BOOLEAN", value: "SDL.joystick.cap.rgb_led", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_JOYSTICK_CAP_PLAYER_LED_BOOLEAN: Property = Property { name: "SDL_PROP_JOYSTICK_CAP_PLAYER_LED_BOOLEAN", value: "SDL.joystick.cap.player_led", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_JOYSTICK_CAP_RUMBLE_BOOLEAN: Property = Property { name: "SDL_PROP_JOYSTICK_CAP_RUMBLE_BOOLEAN", value: "SDL.joystick.cap.rumble", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_JOYSTICK_CAP_TRIGGER_RUMBLE_BOOLEAN: Property = Property { name: "SDL_PROP_JOYSTICK_CAP_TRIGGER_RUMBLE_BOOLEAN", value: "SDL.joystick.cap.trigger_rumble", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_TEXTINPUT_TYPE_NUMBER: Property = Property { name: "SDL_PROP_TEXTINPUT_TYPE_NUMBER", value: "SDL.textinput.type", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_TEXTINPUT_CAPITALIZATION_NUMBER: Property = Property { name: "SDL_PROP_TEXTINPUT_CAPITALIZATION_NUMBER", value: "SDL.textinput.capitalization", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_TEXTINPUT_AUTOCORRECT_BOOLEAN: Property = Property { name: "SDL_PROP_TEXTINPUT_AUTOCORRECT_BOOLEAN", value: "SDL.textinput.autocorrect", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_TEXTINPUT_MULTILINE_BOOLEAN: Property = Property { name: "SDL_PROP_TEXTINPUT_MULTILINE_BOOLEAN", value: "SDL.textinput.multiline", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_TEXTINPUT_TITLE_STRING: Property = Property { name: "SDL_PROP_TEXTINPUT_TITLE_STRING", value: "SDL.textinput.title", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_TEXTINPUT_PLACEHOLDER_STRING: Property = Property { name: "SDL_PROP_TEXTINPUT_PLACEHOLDER_STRING", value: "SDL.textinput.placeholder", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_TEXTINPUT_DEFAULT_TEXT_STRING: Property = Property { name: "SDL_PROP_TEXTINPUT_DEFAULT_TEXT_STRING", value: "SDL.textinput.default_text", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_TEXTINPUT_MAX_LENGTH_NUMBER: Property = Property { name: "SDL_PROP_TEXTINPUT_MAX_LENGTH_NUMBER", value: "SDL.textinput.max_length", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_TEXTINPUT_ANDROID_INPUTTYPE_NUMBER: Property = Property { name: "SDL_PROP_TEXTINPUT_ANDROID_INPUTTYPE_NUMBER", value: "SDL.textinput.android.inputtype", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_PROCESS_CREATE_ARGS_POINTER: Property = Property { name: "SDL_PROP_PROCESS_CREATE_ARGS_POINTER", value: "SDL.process.create.args", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_PROCESS_CREATE_ENVIRONMENT_POINTER: Property = Property { name: "SDL_PROP_PROCESS_CREATE_ENVIRONMENT_POINTER", value: "SDL.process.create.environment", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_PROCESS_CREATE_WORKING_DIRECTORY_STRING: Property = Property { name: "SDL_PROP_PROCESS_CREATE_WORKING_DIRECTORY_STRING", value: "SDL.process.create.working_directory", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_PROCESS_CREATE_STDIN_NUMBER: Property = Property { name: "SDL_PROP_PROCESS_CREATE_STDIN_NUMBER", value: "SDL.process.create.stdin_option", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_PROCESS_CREATE_STDIN_POINTER: Property = Property { name: "SDL_PROP_PROCESS_CREATE_STDIN_POINTER", value: "SDL.process.create.stdin_source", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_PROCESS_CREATE_STDOUT_NUMBER: Property = Property { name: "SDL_PROP_PROCESS_CREATE_STDOUT_NUMBER", value: "SDL.process.create.stdout_option", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_PROCESS_CREATE_STDOUT_POINTER: Property = Property { name: "SDL_PROP_PROCESS_CREATE_STDOUT_POINTER", value: "SDL.process.create.stdout_source", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_PROCESS_CREATE_STDERR_NUMBER: Property = Property { name: "SDL_PROP_PROCESS_CREATE_STDERR_NUMBER", value: "SDL.process.create.stderr_option", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_PROCESS_CREATE_STDERR_POINTER: Property = Property { name: "SDL_PROP_PROCESS_CREATE_STDERR_POINTER", value: "SDL.process.create.stderr_source", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_PROCESS_CREATE_STDERR_TO_STDOUT_BOOLEAN: Property = Property { name: "SDL_PROP_PROCESS_CREATE_STDERR_TO_STDOUT_BOOLEAN", value: "SDL.process.create.stderr_to_stdout", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_PROCESS_CREATE_BACKGROUND_BOOLEAN: Property = Property { name: "SDL_PROP_PROCESS_CREATE_BACKGROUND_BOOLEAN", value: "SDL.process.create.background", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_PROCESS_CREATE_CMDLINE_STRING: Property = Property { name: "SDL_PROP_PROCESS_CREATE_CMDLINE_STRING", value: "SDL.process.create.cmdline", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_PROCESS_PID_NUMBER: Property = Property { name: "SDL_PROP_PROCESS_PID_NUMBER", value: "SDL.process.pid", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_PROCESS_STDIN_POINTER: Property = Property { name: "SDL_PROP_PROCESS_STDIN_POINTER", value: "SDL.process.stdin", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_PROCESS_STDOUT_POINTER: Property = Property { name: "SDL_PROP_PROCESS_STDOUT_POINTER", value: "SDL.process.stdout", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_PROCESS_STDERR_POINTER: Property = Property { name: "SDL_PROP_PROCESS_STDERR_POINTER", value: "SDL.process.stderr", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_PROCESS_BACKGROUND_BOOLEAN: Property = Property { name: "SDL_PROP_PROCESS_BACKGROUND_BOOLEAN", value: "SDL.process.background", ty: PropertyType::Boolean, doc: "" };
/**
* A generic property for naming things.
*
* This property is intended to be added to any SDL_PropertiesID that needs a
* generic name associated with the property set. It is not guaranteed that
* any property set will include this key, but it is convenient to have a
* standard key that any piece of code could reasonably agree to use.
*
* For example, the properties associated with an SDL_Texture might have a
* name string of "player sprites", or an SDL_AudioStream might have
* "background music", etc. This might also be useful for an SDL_IOStream to
* list the path to its asset.
*
* There is no format for the value set with this key; it is expected to be
* human-readable and informational in nature, possibly for logging or
* debugging purposes.
*
* SDL does not currently set this property on any objects it creates, but
* this may change in later versions; it is currently expected that apps and
* external libraries will take advantage of it, when appropriate.
*
* \since This macro is available since SDL 3.4.0.
*/
pub const SDL_PROP_NAME_STRING: Property = Property { name: "SDL_PROP_NAME_STRING", value: "SDL.name", ty: PropertyType::String, doc: "/**\n* A generic property for naming things.\n*\n* This property is intended to be added to any SDL_PropertiesID that needs a\n* generic name associated with the property set. It is not guaranteed that\n* any property set will include this key, but it is convenient to have a\n* standard key that any piece of code could reasonably agree to use.\n*\n* For example, the properties associated with an SDL_Texture might have a\n* name string of \"player sprites\", or an SDL_AudioStream might have\n* \"background music\", etc. This might also be useful for an SDL_IOStream to\n* list the path to its asset.\n*\n* There is no format for the value set with this key; it is expected to be\n* human-readable and informational in nature, possibly for logging or\n* debugging purposes.\n*\n* SDL does not currently set this property on any objects it creates, but\n* this may change in later versions; it is currently expected that apps and\n* external libraries will take advantage of it, when appropriate.\n*\n* \\since This macro is available since SDL 3.4.0.\n*/\n" };
pub const SDL_PROP_RENDERER_CREATE_NAME_STRING: Property = Property { name: "SDL_PROP_RENDERER_CREATE_NAME_STRING", value: "SDL.renderer.create.name", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_RENDERER_CREATE_WINDOW_POINTER: Property = Property { name: "SDL_PROP_RENDERER_CREATE_WINDOW_POINTER", value: "SDL.renderer.create.window", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_RENDERER_CREATE_SURFACE_POINTER: Property = Property { name: "SDL_PROP_RENDERER_CREATE_SURFACE_POINTER", value: "SDL.renderer.create.surface", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_RENDERER_CREATE_OUTPUT_COLORSPACE_NUMBER: Property = Property { name: "SDL_PROP_RENDERER_CREATE_OUTPUT_COLORSPACE_NUMBER", value: "SDL.renderer.create.output_colorspace", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_RENDERER_CREATE_PRESENT_VSYNC_NUMBER: Property = Property { name: "SDL_PROP_RENDERER_CREATE_PRESENT_VSYNC_NUMBER", value: "SDL.renderer.create.present_vsync", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_RENDERER_CREATE_GPU_DEVICE_POINTER: Property = Property { name: "SDL_PROP_RENDERER_CREATE_GPU_DEVICE_POINTER", value: "SDL.renderer.create.gpu.device", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_RENDERER_CREATE_GPU_SHADERS_SPIRV_BOOLEAN: Property = Property { name: "SDL_PROP_RENDERER_CREATE_GPU_SHADERS_SPIRV_BOOLEAN", value: "SDL.renderer.create.gpu.shaders_spirv", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_RENDERER_CREATE_GPU_SHADERS_DXIL_BOOLEAN: Property = Property { name: "SDL_PROP_RENDERER_CREATE_GPU_SHADERS_DXIL_BOOLEAN", value: "SDL.renderer.create.gpu.shaders_dxil", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_RENDERER_CREATE_GPU_SHADERS_MSL_BOOLEAN: Property = Property { name: "SDL_PROP_RENDERER_CREATE_GPU_SHADERS_MSL_BOOLEAN", value: "SDL.renderer.create.gpu.shaders_msl", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_RENDERER_CREATE_VULKAN_INSTANCE_POINTER: Property = Property { name: "SDL_PROP_RENDERER_CREATE_VULKAN_INSTANCE_POINTER", value: "SDL.renderer.create.vulkan.instance", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_RENDERER_CREATE_VULKAN_SURFACE_NUMBER: Property = Property { name: "SDL_PROP_RENDERER_CREATE_VULKAN_SURFACE_NUMBER", value: "SDL.renderer.create.vulkan.surface", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_RENDERER_CREATE_VULKAN_PHYSICAL_DEVICE_POINTER: Property = Property { name: "SDL_PROP_RENDERER_CREATE_VULKAN_PHYSICAL_DEVICE_POINTER", value: "SDL.renderer.create.vulkan.physical_device", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_RENDERER_CREATE_VULKAN_DEVICE_POINTER: Property = Property { name: "SDL_PROP_RENDERER_CREATE_VULKAN_DEVICE_POINTER", value: "SDL.renderer.create.vulkan.device", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_RENDERER_CREATE_VULKAN_GRAPHICS_QUEUE_FAMILY_INDEX_NUMBER: Property = Property { name: "SDL_PROP_RENDERER_CREATE_VULKAN_GRAPHICS_QUEUE_FAMILY_INDEX_NUMBER", value: "SDL.renderer.create.vulkan.graphics_queue_family_index", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_RENDERER_CREATE_VULKAN_PRESENT_QUEUE_FAMILY_INDEX_NUMBER: Property = Property { name: "SDL_PROP_RENDERER_CREATE_VULKAN_PRESENT_QUEUE_FAMILY_INDEX_NUMBER", value: "SDL.renderer.create.vulkan.present_queue_family_index", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_RENDERER_NAME_STRING: Property = Property { name: "SDL_PROP_RENDERER_NAME_STRING", value: "SDL.renderer.name", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_RENDERER_WINDOW_POINTER: Property = Property { name: "SDL_PROP_RENDERER_WINDOW_POINTER", value: "SDL.renderer.window", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_RENDERER_SURFACE_POINTER: Property = Property { name: "SDL_PROP_RENDERER_SURFACE_POINTER", value: "SDL.renderer.surface", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_RENDERER_VSYNC_NUMBER: Property = Property { name: "SDL_PROP_RENDERER_VSYNC_NUMBER", value: "SDL.renderer.vsync", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_RENDERER_MAX_TEXTURE_SIZE_NUMBER: Property = Property { name: "SDL_PROP_RENDERER_MAX_TEXTURE_SIZE_NUMBER", value: "SDL.renderer.max_texture_size", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_RENDERER_TEXTURE_FORMATS_POINTER: Property = Property { name: "SDL_PROP_RENDERER_TEXTURE_FORMATS_POINTER", value: "SDL.renderer.texture_formats", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_RENDERER_TEXTURE_WRAPPING_BOOLEAN: Property = Property { name: "SDL_PROP_RENDERER_TEXTURE_WRAPPING_BOOLEAN", value: "SDL.renderer.texture_wrapping", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_RENDERER_OUTPUT_COLORSPACE_NUMBER: Property = Property { name: "SDL_PROP_RENDERER_OUTPUT_COLORSPACE_NUMBER", value: "SDL.renderer.output_colorspace", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_RENDERER_HDR_ENABLED_BOOLEAN: Property = Property { name: "SDL_PROP_RENDERER_HDR_ENABLED_BOOLEAN", value: "SDL.renderer.HDR_enabled", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_RENDERER_SDR_WHITE_POINT_FLOAT: Property = Property { name: "SDL_PROP_RENDERER_SDR_WHITE_POINT_FLOAT", value: "SDL.renderer.SDR_white_point", ty: PropertyType::Float, doc: "" };
pub const SDL_PROP_RENDERER_HDR_HEADROOM_FLOAT: Property = Property { name: "SDL_PROP_RENDERER_HDR_HEADROOM_FLOAT", value: "SDL.renderer.HDR_headroom", ty: PropertyType::Float, doc: "" };
pub const SDL_PROP_RENDERER_D3D9_DEVICE_POINTER: Property = Property { name: "SDL_PROP_RENDERER_D3D9_DEVICE_POINTER", value: "SDL.renderer.d3d9.device", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_RENDERER_D3D11_DEVICE_POINTER: Property = Property { name: "SDL_PROP_RENDERER_D3D11_DEVICE_POINTER", value: "SDL.renderer.d3d11.device", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_RENDERER_D3D11_SWAPCHAIN_POINTER: Property = Property { name: "SDL_PROP_RENDERER_D3D11_SWAPCHAIN_POINTER", value: "SDL.renderer.d3d11.swap_chain", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_RENDERER_D3D12_DEVICE_POINTER: Property = Property { name: "SDL_PROP_RENDERER_D3D12_DEVICE_POINTER", value: "SDL.renderer.d3d12.device", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_RENDERER_D3D12_SWAPCHAIN_POINTER: Property = Property { name: "SDL_PROP_RENDERER_D3D12_SWAPCHAIN_POINTER", value: "SDL.renderer.d3d12.swap_chain", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_RENDERER_D3D12_COMMAND_QUEUE_POINTER: Property = Property { name: "SDL_PROP_RENDERER_D3D12_COMMAND_QUEUE_POINTER", value: "SDL.renderer.d3d12.command_queue", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_RENDERER_VULKAN_INSTANCE_POINTER: Property = Property { name: "SDL_PROP_RENDERER_VULKAN_INSTANCE_POINTER", value: "SDL.renderer.vulkan.instance", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_RENDERER_VULKAN_SURFACE_NUMBER: Property = Property { name: "SDL_PROP_RENDERER_VULKAN_SURFACE_NUMBER", value: "SDL.renderer.vulkan.surface", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_RENDERER_VULKAN_PHYSICAL_DEVICE_POINTER: Property = Property { name: "SDL_PROP_RENDERER_VULKAN_PHYSICAL_DEVICE_POINTER", value: "SDL.renderer.vulkan.physical_device", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_RENDERER_VULKAN_DEVICE_POINTER: Property = Property { name: "SDL_PROP_RENDERER_VULKAN_DEVICE_POINTER", value: "SDL.renderer.vulkan.device", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_RENDERER_VULKAN_GRAPHICS_QUEUE_FAMILY_INDEX_NUMBER: Property = Property { name: "SDL_PROP_RENDERER_VULKAN_GRAPHICS_QUEUE_FAMILY_INDEX_NUMBER", value: "SDL.renderer.vulkan.graphics_queue_family_index", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_RENDERER_VULKAN_PRESENT_QUEUE_FAMILY_INDEX_NUMBER: Property = Property { name: "SDL_PROP_RENDERER_VULKAN_PRESENT_QUEUE_FAMILY_INDEX_NUMBER", value: "SDL.renderer.vulkan.present_queue_family_index", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_RENDERER_VULKAN_SWAPCHAIN_IMAGE_COUNT_NUMBER: Property = Property { name: "SDL_PROP_RENDERER_VULKAN_SWAPCHAIN_IMAGE_COUNT_NUMBER", value: "SDL.renderer.vulkan.swapchain_image_count", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_RENDERER_GPU_DEVICE_POINTER: Property = Property { name: "SDL_PROP_RENDERER_GPU_DEVICE_POINTER", value: "SDL.renderer.gpu.device", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_TEXTURE_CREATE_COLORSPACE_NUMBER: Property = Property { name: "SDL_PROP_TEXTURE_CREATE_COLORSPACE_NUMBER", value: "SDL.texture.create.colorspace", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_TEXTURE_CREATE_FORMAT_NUMBER: Property = Property { name: "SDL_PROP_TEXTURE_CREATE_FORMAT_NUMBER", value: "SDL.texture.create.format", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_TEXTURE_CREATE_ACCESS_NUMBER: Property = Property { name: "SDL_PROP_TEXTURE_CREATE_ACCESS_NUMBER", value: "SDL.texture.create.access", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_TEXTURE_CREATE_WIDTH_NUMBER: Property = Property { name: "SDL_PROP_TEXTURE_CREATE_WIDTH_NUMBER", value: "SDL.texture.create.width", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_TEXTURE_CREATE_HEIGHT_NUMBER: Property = Property { name: "SDL_PROP_TEXTURE_CREATE_HEIGHT_NUMBER", value: "SDL.texture.create.height", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_TEXTURE_CREATE_PALETTE_POINTER: Property = Property { name: "SDL_PROP_TEXTURE_CREATE_PALETTE_POINTER", value: "SDL.texture.create.palette", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_TEXTURE_CREATE_SDR_WHITE_POINT_FLOAT: Property = Property { name: "SDL_PROP_TEXTURE_CREATE_SDR_WHITE_POINT_FLOAT", value: "SDL.texture.create.SDR_white_point", ty: PropertyType::Float, doc: "" };
pub const SDL_PROP_TEXTURE_CREATE_HDR_HEADROOM_FLOAT: Property = Property { name: "SDL_PROP_TEXTURE_CREATE_HDR_HEADROOM_FLOAT", value: "SDL.texture.create.HDR_headroom", ty: PropertyType::Float, doc: "" };
pub const SDL_PROP_TEXTURE_CREATE_D3D11_TEXTURE_POINTER: Property = Property { name: "SDL_PROP_TEXTURE_CREATE_D3D11_TEXTURE_POINTER", value: "SDL.texture.create.d3d11.texture", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_TEXTURE_CREATE_D3D11_TEXTURE_U_POINTER: Property = Property { name: "SDL_PROP_TEXTURE_CREATE_D3D11_TEXTURE_U_POINTER", value: "SDL.texture.create.d3d11.texture_u", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_TEXTURE_CREATE_D3D11_TEXTURE_V_POINTER: Property = Property { name: "SDL_PROP_TEXTURE_CREATE_D3D11_TEXTURE_V_POINTER", value: "SDL.texture.create.d3d11.texture_v", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_TEXTURE_CREATE_D3D12_TEXTURE_POINTER: Property = Property { name: "SDL_PROP_TEXTURE_CREATE_D3D12_TEXTURE_POINTER", value: "SDL.texture.create.d3d12.texture", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_TEXTURE_CREATE_D3D12_TEXTURE_U_POINTER: Property = Property { name: "SDL_PROP_TEXTURE_CREATE_D3D12_TEXTURE_U_POINTER", value: "SDL.texture.create.d3d12.texture_u", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_TEXTURE_CREATE_D3D12_TEXTURE_V_POINTER: Property = Property { name: "SDL_PROP_TEXTURE_CREATE_D3D12_TEXTURE_V_POINTER", value: "SDL.texture.create.d3d12.texture_v", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_TEXTURE_CREATE_METAL_PIXELBUFFER_POINTER: Property = Property { name: "SDL_PROP_TEXTURE_CREATE_METAL_PIXELBUFFER_POINTER", value: "SDL.texture.create.metal.pixelbuffer", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_TEXTURE_CREATE_OPENGL_TEXTURE_NUMBER: Property = Property { name: "SDL_PROP_TEXTURE_CREATE_OPENGL_TEXTURE_NUMBER", value: "SDL.texture.create.opengl.texture", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_TEXTURE_CREATE_OPENGL_TEXTURE_UV_NUMBER: Property = Property { name: "SDL_PROP_TEXTURE_CREATE_OPENGL_TEXTURE_UV_NUMBER", value: "SDL.texture.create.opengl.texture_uv", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_TEXTURE_CREATE_OPENGL_TEXTURE_U_NUMBER: Property = Property { name: "SDL_PROP_TEXTURE_CREATE_OPENGL_TEXTURE_U_NUMBER", value: "SDL.texture.create.opengl.texture_u", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_TEXTURE_CREATE_OPENGL_TEXTURE_V_NUMBER: Property = Property { name: "SDL_PROP_TEXTURE_CREATE_OPENGL_TEXTURE_V_NUMBER", value: "SDL.texture.create.opengl.texture_v", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_TEXTURE_CREATE_OPENGLES2_TEXTURE_NUMBER: Property = Property { name: "SDL_PROP_TEXTURE_CREATE_OPENGLES2_TEXTURE_NUMBER", value: "SDL.texture.create.opengles2.texture", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_TEXTURE_CREATE_OPENGLES2_TEXTURE_UV_NUMBER: Property = Property { name: "SDL_PROP_TEXTURE_CREATE_OPENGLES2_TEXTURE_UV_NUMBER", value: "SDL.texture.create.opengles2.texture_uv", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_TEXTURE_CREATE_OPENGLES2_TEXTURE_U_NUMBER: Property = Property { name: "SDL_PROP_TEXTURE_CREATE_OPENGLES2_TEXTURE_U_NUMBER", value: "SDL.texture.create.opengles2.texture_u", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_TEXTURE_CREATE_OPENGLES2_TEXTURE_V_NUMBER: Property = Property { name: "SDL_PROP_TEXTURE_CREATE_OPENGLES2_TEXTURE_V_NUMBER", value: "SDL.texture.create.opengles2.texture_v", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_TEXTURE_CREATE_VULKAN_TEXTURE_NUMBER: Property = Property { name: "SDL_PROP_TEXTURE_CREATE_VULKAN_TEXTURE_NUMBER", value: "SDL.texture.create.vulkan.texture", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_TEXTURE_CREATE_VULKAN_LAYOUT_NUMBER: Property = Property { name: "SDL_PROP_TEXTURE_CREATE_VULKAN_LAYOUT_NUMBER", value: "SDL.texture.create.vulkan.layout", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_TEXTURE_CREATE_GPU_TEXTURE_POINTER: Property = Property { name: "SDL_PROP_TEXTURE_CREATE_GPU_TEXTURE_POINTER", value: "SDL.texture.create.gpu.texture", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_TEXTURE_CREATE_GPU_TEXTURE_UV_POINTER: Property = Property { name: "SDL_PROP_TEXTURE_CREATE_GPU_TEXTURE_UV_POINTER", value: "SDL.texture.create.gpu.texture_uv", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_TEXTURE_CREATE_GPU_TEXTURE_U_POINTER: Property = Property { name: "SDL_PROP_TEXTURE_CREATE_GPU_TEXTURE_U_POINTER", value: "SDL.texture.create.gpu.texture_u", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_TEXTURE_CREATE_GPU_TEXTURE_V_POINTER: Property = Property { name: "SDL_PROP_TEXTURE_CREATE_GPU_TEXTURE_V_POINTER", value: "SDL.texture.create.gpu.texture_v", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_TEXTURE_COLORSPACE_NUMBER: Property = Property { name: "SDL_PROP_TEXTURE_COLORSPACE_NUMBER", value: "SDL.texture.colorspace", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_TEXTURE_FORMAT_NUMBER: Property = Property { name: "SDL_PROP_TEXTURE_FORMAT_NUMBER", value: "SDL.texture.format", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_TEXTURE_ACCESS_NUMBER: Property = Property { name: "SDL_PROP_TEXTURE_ACCESS_NUMBER", value: "SDL.texture.access", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_TEXTURE_WIDTH_NUMBER: Property = Property { name: "SDL_PROP_TEXTURE_WIDTH_NUMBER", value: "SDL.texture.width", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_TEXTURE_HEIGHT_NUMBER: Property = Property { name: "SDL_PROP_TEXTURE_HEIGHT_NUMBER", value: "SDL.texture.height", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_TEXTURE_SDR_WHITE_POINT_FLOAT: Property = Property { name: "SDL_PROP_TEXTURE_SDR_WHITE_POINT_FLOAT", value: "SDL.texture.SDR_white_point", ty: PropertyType::Float, doc: "" };
pub const SDL_PROP_TEXTURE_HDR_HEADROOM_FLOAT: Property = Property { name: "SDL_PROP_TEXTURE_HDR_HEADROOM_FLOAT", value: "SDL.texture.HDR_headroom", ty: PropertyType::Float, doc: "" };
pub const SDL_PROP_TEXTURE_D3D11_TEXTURE_POINTER: Property = Property { name: "SDL_PROP_TEXTURE_D3D11_TEXTURE_POINTER", value: "SDL.texture.d3d11.texture", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_TEXTURE_D3D11_TEXTURE_U_POINTER: Property = Property { name: "SDL_PROP_TEXTURE_D3D11_TEXTURE_U_POINTER", value: "SDL.texture.d3d11.texture_u", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_TEXTURE_D3D11_TEXTURE_V_POINTER: Property = Property { name: "SDL_PROP_TEXTURE_D3D11_TEXTURE_V_POINTER", value: "SDL.texture.d3d11.texture_v", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_TEXTURE_D3D12_TEXTURE_POINTER: Property = Property { name: "SDL_PROP_TEXTURE_D3D12_TEXTURE_POINTER", value: "SDL.texture.d3d12.texture", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_TEXTURE_D3D12_TEXTURE_U_POINTER: Property = Property { name: "SDL_PROP_TEXTURE_D3D12_TEXTURE_U_POINTER", value: "SDL.texture.d3d12.texture_u", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_TEXTURE_D3D12_TEXTURE_V_POINTER: Property = Property { name: "SDL_PROP_TEXTURE_D3D12_TEXTURE_V_POINTER", value: "SDL.texture.d3d12.texture_v", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_TEXTURE_OPENGL_TEXTURE_NUMBER: Property = Property { name: "SDL_PROP_TEXTURE_OPENGL_TEXTURE_NUMBER", value: "SDL.texture.opengl.texture", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_TEXTURE_OPENGL_TEXTURE_UV_NUMBER: Property = Property { name: "SDL_PROP_TEXTURE_OPENGL_TEXTURE_UV_NUMBER", value: "SDL.texture.opengl.texture_uv", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_TEXTURE_OPENGL_TEXTURE_U_NUMBER: Property = Property { name: "SDL_PROP_TEXTURE_OPENGL_TEXTURE_U_NUMBER", value: "SDL.texture.opengl.texture_u", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_TEXTURE_OPENGL_TEXTURE_V_NUMBER: Property = Property { name: "SDL_PROP_TEXTURE_OPENGL_TEXTURE_V_NUMBER", value: "SDL.texture.opengl.texture_v", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_TEXTURE_OPENGL_TEXTURE_TARGET_NUMBER: Property = Property { name: "SDL_PROP_TEXTURE_OPENGL_TEXTURE_TARGET_NUMBER", value: "SDL.texture.opengl.target", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_TEXTURE_OPENGL_TEX_W_FLOAT: Property = Property { name: "SDL_PROP_TEXTURE_OPENGL_TEX_W_FLOAT", value: "SDL.texture.opengl.tex_w", ty: PropertyType::Float, doc: "" };
pub const SDL_PROP_TEXTURE_OPENGL_TEX_H_FLOAT: Property = Property { name: "SDL_PROP_TEXTURE_OPENGL_TEX_H_FLOAT", value: "SDL.texture.opengl.tex_h", ty: PropertyType::Float, doc: "" };
pub const SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_NUMBER: Property = Property { name: "SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_NUMBER", value: "SDL.texture.opengles2.texture", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_UV_NUMBER: Property = Property { name: "SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_UV_NUMBER", value: "SDL.texture.opengles2.texture_uv", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_U_NUMBER: Property = Property { name: "SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_U_NUMBER", value: "SDL.texture.opengles2.texture_u", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_V_NUMBER: Property = Property { name: "SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_V_NUMBER", value: "SDL.texture.opengles2.texture_v", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_TARGET_NUMBER: Property = Property { name: "SDL_PROP_TEXTURE_OPENGLES2_TEXTURE_TARGET_NUMBER", value: "SDL.texture.opengles2.target", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_TEXTURE_VULKAN_TEXTURE_NUMBER: Property = Property { name: "SDL_PROP_TEXTURE_VULKAN_TEXTURE_NUMBER", value: "SDL.texture.vulkan.texture", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_TEXTURE_GPU_TEXTURE_POINTER: Property = Property { name: "SDL_PROP_TEXTURE_GPU_TEXTURE_POINTER", value: "SDL.texture.gpu.texture", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_TEXTURE_GPU_TEXTURE_UV_POINTER: Property = Property { name: "SDL_PROP_TEXTURE_GPU_TEXTURE_UV_POINTER", value: "SDL.texture.gpu.texture_uv", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_TEXTURE_GPU_TEXTURE_U_POINTER: Property = Property { name: "SDL_PROP_TEXTURE_GPU_TEXTURE_U_POINTER", value: "SDL.texture.gpu.texture_u", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_TEXTURE_GPU_TEXTURE_V_POINTER: Property = Property { name: "SDL_PROP_TEXTURE_GPU_TEXTURE_V_POINTER", value: "SDL.texture.gpu.texture_v", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_SURFACE_SDR_WHITE_POINT_FLOAT: Property = Property { name: "SDL_PROP_SURFACE_SDR_WHITE_POINT_FLOAT", value: "SDL.surface.SDR_white_point", ty: PropertyType::Float, doc: "" };
pub const SDL_PROP_SURFACE_HDR_HEADROOM_FLOAT: Property = Property { name: "SDL_PROP_SURFACE_HDR_HEADROOM_FLOAT", value: "SDL.surface.HDR_headroom", ty: PropertyType::Float, doc: "" };
pub const SDL_PROP_SURFACE_TONEMAP_OPERATOR_STRING: Property = Property { name: "SDL_PROP_SURFACE_TONEMAP_OPERATOR_STRING", value: "SDL.surface.tonemap", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_SURFACE_HOTSPOT_X_NUMBER: Property = Property { name: "SDL_PROP_SURFACE_HOTSPOT_X_NUMBER", value: "SDL.surface.hotspot.x", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_SURFACE_HOTSPOT_Y_NUMBER: Property = Property { name: "SDL_PROP_SURFACE_HOTSPOT_Y_NUMBER", value: "SDL.surface.hotspot.y", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_SURFACE_ROTATION_FLOAT: Property = Property { name: "SDL_PROP_SURFACE_ROTATION_FLOAT", value: "SDL.surface.rotation", ty: PropertyType::Float, doc: "" };
pub const SDL_PROP_THREAD_CREATE_ENTRY_FUNCTION_POINTER: Property = Property { name: "SDL_PROP_THREAD_CREATE_ENTRY_FUNCTION_POINTER", value: "SDL.thread.create.entry_function", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_THREAD_CREATE_NAME_STRING: Property = Property { name: "SDL_PROP_THREAD_CREATE_NAME_STRING", value: "SDL.thread.create.name", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_THREAD_CREATE_USERDATA_POINTER: Property = Property { name: "SDL_PROP_THREAD_CREATE_USERDATA_POINTER", value: "SDL.thread.create.userdata", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_THREAD_CREATE_STACKSIZE_NUMBER: Property = Property { name: "SDL_PROP_THREAD_CREATE_STACKSIZE_NUMBER", value: "SDL.thread.create.stacksize", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_TRAY_CREATE_ICON_POINTER: Property = Property { name: "SDL_PROP_TRAY_CREATE_ICON_POINTER", value: "SDL.tray.create.icon", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_TRAY_CREATE_TOOLTIP_STRING: Property = Property { name: "SDL_PROP_TRAY_CREATE_TOOLTIP_STRING", value: "SDL.tray.create.tooltip", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_TRAY_CREATE_USERDATA_POINTER: Property = Property { name: "SDL_PROP_TRAY_CREATE_USERDATA_POINTER", value: "SDL.tray.create.userdata", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_TRAY_CREATE_LEFTCLICK_CALLBACK_POINTER: Property = Property { name: "SDL_PROP_TRAY_CREATE_LEFTCLICK_CALLBACK_POINTER", value: "SDL.tray.create.leftclick_callback", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_TRAY_CREATE_RIGHTCLICK_CALLBACK_POINTER: Property = Property { name: "SDL_PROP_TRAY_CREATE_RIGHTCLICK_CALLBACK_POINTER", value: "SDL.tray.create.rightclick_callback", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_TRAY_CREATE_MIDDLECLICK_CALLBACK_POINTER: Property = Property { name: "SDL_PROP_TRAY_CREATE_MIDDLECLICK_CALLBACK_POINTER", value: "SDL.tray.create.middleclick_callback", ty: PropertyType::Pointer, doc: "" };
/**
* The pointer to the global `wl_display` object used by the Wayland video
* backend.
*
* Can be set before the video subsystem is initialized to import an external
* `wl_display` object from an application or toolkit for use in SDL, or read
* after initialization to export the `wl_display` used by the Wayland video
* backend. Setting this property after the video subsystem has been
* initialized has no effect, and reading it when the video subsystem is
* uninitialized will either return the user provided value, if one was set
* prior to initialization, or NULL. See docs/README-wayland.md for more
* information.
*
* \since This macro is available since SDL 3.2.0.
*/
pub const SDL_PROP_GLOBAL_VIDEO_WAYLAND_WL_DISPLAY_POINTER: Property = Property { name: "SDL_PROP_GLOBAL_VIDEO_WAYLAND_WL_DISPLAY_POINTER", value: "SDL.video.wayland.wl_display", ty: PropertyType::Pointer, doc: "/**\n* The pointer to the global `wl_display` object used by the Wayland video\n* backend.\n*\n* Can be set before the video subsystem is initialized to import an external\n* `wl_display` object from an application or toolkit for use in SDL, or read\n* after initialization to export the `wl_display` used by the Wayland video\n* backend. Setting this property after the video subsystem has been\n* initialized has no effect, and reading it when the video subsystem is\n* uninitialized will either return the user provided value, if one was set\n* prior to initialization, or NULL. See docs/README-wayland.md for more\n* information.\n*\n* \\since This macro is available since SDL 3.2.0.\n*/\n" };
pub const SDL_PROP_DISPLAY_HDR_ENABLED_BOOLEAN: Property = Property { name: "SDL_PROP_DISPLAY_HDR_ENABLED_BOOLEAN", value: "SDL.display.HDR_enabled", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_DISPLAY_KMSDRM_PANEL_ORIENTATION_NUMBER: Property = Property { name: "SDL_PROP_DISPLAY_KMSDRM_PANEL_ORIENTATION_NUMBER", value: "SDL.display.KMSDRM.panel_orientation", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_DISPLAY_WAYLAND_WL_OUTPUT_POINTER: Property = Property { name: "SDL_PROP_DISPLAY_WAYLAND_WL_OUTPUT_POINTER", value: "SDL.display.wayland.wl_output", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_DISPLAY_WINDOWS_HMONITOR_POINTER: Property = Property { name: "SDL_PROP_DISPLAY_WINDOWS_HMONITOR_POINTER", value: "SDL.display.windows.hmonitor", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_WINDOW_CREATE_ALWAYS_ON_TOP_BOOLEAN: Property = Property { name: "SDL_PROP_WINDOW_CREATE_ALWAYS_ON_TOP_BOOLEAN", value: "SDL.window.create.always_on_top", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_WINDOW_CREATE_BORDERLESS_BOOLEAN: Property = Property { name: "SDL_PROP_WINDOW_CREATE_BORDERLESS_BOOLEAN", value: "SDL.window.create.borderless", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_WINDOW_CREATE_CONSTRAIN_POPUP_BOOLEAN: Property = Property { name: "SDL_PROP_WINDOW_CREATE_CONSTRAIN_POPUP_BOOLEAN", value: "SDL.window.create.constrain_popup", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_WINDOW_CREATE_FOCUSABLE_BOOLEAN: Property = Property { name: "SDL_PROP_WINDOW_CREATE_FOCUSABLE_BOOLEAN", value: "SDL.window.create.focusable", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_WINDOW_CREATE_EXTERNAL_GRAPHICS_CONTEXT_BOOLEAN: Property = Property { name: "SDL_PROP_WINDOW_CREATE_EXTERNAL_GRAPHICS_CONTEXT_BOOLEAN", value: "SDL.window.create.external_graphics_context", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_WINDOW_CREATE_FLAGS_NUMBER: Property = Property { name: "SDL_PROP_WINDOW_CREATE_FLAGS_NUMBER", value: "SDL.window.create.flags", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_WINDOW_CREATE_FULLSCREEN_BOOLEAN: Property = Property { name: "SDL_PROP_WINDOW_CREATE_FULLSCREEN_BOOLEAN", value: "SDL.window.create.fullscreen", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_WINDOW_CREATE_HEIGHT_NUMBER: Property = Property { name: "SDL_PROP_WINDOW_CREATE_HEIGHT_NUMBER", value: "SDL.window.create.height", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_WINDOW_CREATE_HIDDEN_BOOLEAN: Property = Property { name: "SDL_PROP_WINDOW_CREATE_HIDDEN_BOOLEAN", value: "SDL.window.create.hidden", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_WINDOW_CREATE_HIGH_PIXEL_DENSITY_BOOLEAN: Property = Property { name: "SDL_PROP_WINDOW_CREATE_HIGH_PIXEL_DENSITY_BOOLEAN", value: "SDL.window.create.high_pixel_density", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_WINDOW_CREATE_MAXIMIZED_BOOLEAN: Property = Property { name: "SDL_PROP_WINDOW_CREATE_MAXIMIZED_BOOLEAN", value: "SDL.window.create.maximized", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_WINDOW_CREATE_MENU_BOOLEAN: Property = Property { name: "SDL_PROP_WINDOW_CREATE_MENU_BOOLEAN", value: "SDL.window.create.menu", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_WINDOW_CREATE_METAL_BOOLEAN: Property = Property { name: "SDL_PROP_WINDOW_CREATE_METAL_BOOLEAN", value: "SDL.window.create.metal", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_WINDOW_CREATE_MINIMIZED_BOOLEAN: Property = Property { name: "SDL_PROP_WINDOW_CREATE_MINIMIZED_BOOLEAN", value: "SDL.window.create.minimized", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_WINDOW_CREATE_MODAL_BOOLEAN: Property = Property { name: "SDL_PROP_WINDOW_CREATE_MODAL_BOOLEAN", value: "SDL.window.create.modal", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_WINDOW_CREATE_MOUSE_GRABBED_BOOLEAN: Property = Property { name: "SDL_PROP_WINDOW_CREATE_MOUSE_GRABBED_BOOLEAN", value: "SDL.window.create.mouse_grabbed", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_WINDOW_CREATE_OPENGL_BOOLEAN: Property = Property { name: "SDL_PROP_WINDOW_CREATE_OPENGL_BOOLEAN", value: "SDL.window.create.opengl", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_WINDOW_CREATE_PARENT_POINTER: Property = Property { name: "SDL_PROP_WINDOW_CREATE_PARENT_POINTER", value: "SDL.window.create.parent", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_WINDOW_CREATE_RESIZABLE_BOOLEAN: Property = Property { name: "SDL_PROP_WINDOW_CREATE_RESIZABLE_BOOLEAN", value: "SDL.window.create.resizable", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_WINDOW_CREATE_TITLE_STRING: Property = Property { name: "SDL_PROP_WINDOW_CREATE_TITLE_STRING", value: "SDL.window.create.title", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_WINDOW_CREATE_TRANSPARENT_BOOLEAN: Property = Property { name: "SDL_PROP_WINDOW_CREATE_TRANSPARENT_BOOLEAN", value: "SDL.window.create.transparent", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_WINDOW_CREATE_TOOLTIP_BOOLEAN: Property = Property { name: "SDL_PROP_WINDOW_CREATE_TOOLTIP_BOOLEAN", value: "SDL.window.create.tooltip", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_WINDOW_CREATE_UTILITY_BOOLEAN: Property = Property { name: "SDL_PROP_WINDOW_CREATE_UTILITY_BOOLEAN", value: "SDL.window.create.utility", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_WINDOW_CREATE_VULKAN_BOOLEAN: Property = Property { name: "SDL_PROP_WINDOW_CREATE_VULKAN_BOOLEAN", value: "SDL.window.create.vulkan", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_WINDOW_CREATE_WIDTH_NUMBER: Property = Property { name: "SDL_PROP_WINDOW_CREATE_WIDTH_NUMBER", value: "SDL.window.create.width", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_WINDOW_CREATE_X_NUMBER: Property = Property { name: "SDL_PROP_WINDOW_CREATE_X_NUMBER", value: "SDL.window.create.x", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_WINDOW_CREATE_Y_NUMBER: Property = Property { name: "SDL_PROP_WINDOW_CREATE_Y_NUMBER", value: "SDL.window.create.y", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_WINDOW_CREATE_COCOA_WINDOW_POINTER: Property = Property { name: "SDL_PROP_WINDOW_CREATE_COCOA_WINDOW_POINTER", value: "SDL.window.create.cocoa.window", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_WINDOW_CREATE_COCOA_VIEW_POINTER: Property = Property { name: "SDL_PROP_WINDOW_CREATE_COCOA_VIEW_POINTER", value: "SDL.window.create.cocoa.view", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_WINDOW_CREATE_WINDOWSCENE_POINTER: Property = Property { name: "SDL_PROP_WINDOW_CREATE_WINDOWSCENE_POINTER", value: "SDL.window.create.uikit.windowscene", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_WINDOW_CREATE_WAYLAND_SURFACE_ROLE_CUSTOM_BOOLEAN: Property = Property { name: "SDL_PROP_WINDOW_CREATE_WAYLAND_SURFACE_ROLE_CUSTOM_BOOLEAN", value: "SDL.window.create.wayland.surface_role_custom", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_WINDOW_CREATE_WAYLAND_CREATE_EGL_WINDOW_BOOLEAN: Property = Property { name: "SDL_PROP_WINDOW_CREATE_WAYLAND_CREATE_EGL_WINDOW_BOOLEAN", value: "SDL.window.create.wayland.create_egl_window", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_WINDOW_CREATE_WAYLAND_WL_SURFACE_POINTER: Property = Property { name: "SDL_PROP_WINDOW_CREATE_WAYLAND_WL_SURFACE_POINTER", value: "SDL.window.create.wayland.wl_surface", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_WINDOW_CREATE_WIN32_HWND_POINTER: Property = Property { name: "SDL_PROP_WINDOW_CREATE_WIN32_HWND_POINTER", value: "SDL.window.create.win32.hwnd", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_WINDOW_CREATE_WIN32_PIXEL_FORMAT_HWND_POINTER: Property = Property { name: "SDL_PROP_WINDOW_CREATE_WIN32_PIXEL_FORMAT_HWND_POINTER", value: "SDL.window.create.win32.pixel_format_hwnd", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_WINDOW_CREATE_X11_WINDOW_NUMBER: Property = Property { name: "SDL_PROP_WINDOW_CREATE_X11_WINDOW_NUMBER", value: "SDL.window.create.x11.window", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_WINDOW_CREATE_EMSCRIPTEN_CANVAS_ID_STRING: Property = Property { name: "SDL_PROP_WINDOW_CREATE_EMSCRIPTEN_CANVAS_ID_STRING", value: "SDL.window.create.emscripten.canvas_id", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_WINDOW_CREATE_EMSCRIPTEN_KEYBOARD_ELEMENT_STRING: Property = Property { name: "SDL_PROP_WINDOW_CREATE_EMSCRIPTEN_KEYBOARD_ELEMENT_STRING", value: "SDL.window.create.emscripten.keyboard_element", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_WINDOW_SHAPE_POINTER: Property = Property { name: "SDL_PROP_WINDOW_SHAPE_POINTER", value: "SDL.window.shape", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_WINDOW_HDR_ENABLED_BOOLEAN: Property = Property { name: "SDL_PROP_WINDOW_HDR_ENABLED_BOOLEAN", value: "SDL.window.HDR_enabled", ty: PropertyType::Boolean, doc: "" };
pub const SDL_PROP_WINDOW_SDR_WHITE_LEVEL_FLOAT: Property = Property { name: "SDL_PROP_WINDOW_SDR_WHITE_LEVEL_FLOAT", value: "SDL.window.SDR_white_level", ty: PropertyType::Float, doc: "" };
pub const SDL_PROP_WINDOW_HDR_HEADROOM_FLOAT: Property = Property { name: "SDL_PROP_WINDOW_HDR_HEADROOM_FLOAT", value: "SDL.window.HDR_headroom", ty: PropertyType::Float, doc: "" };
pub const SDL_PROP_WINDOW_ANDROID_WINDOW_POINTER: Property = Property { name: "SDL_PROP_WINDOW_ANDROID_WINDOW_POINTER", value: "SDL.window.android.window", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_WINDOW_ANDROID_SURFACE_POINTER: Property = Property { name: "SDL_PROP_WINDOW_ANDROID_SURFACE_POINTER", value: "SDL.window.android.surface", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_WINDOW_UIKIT_WINDOW_POINTER: Property = Property { name: "SDL_PROP_WINDOW_UIKIT_WINDOW_POINTER", value: "SDL.window.uikit.window", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_WINDOW_UIKIT_METAL_VIEW_TAG_NUMBER: Property = Property { name: "SDL_PROP_WINDOW_UIKIT_METAL_VIEW_TAG_NUMBER", value: "SDL.window.uikit.metal_view_tag", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_WINDOW_UIKIT_OPENGL_FRAMEBUFFER_NUMBER: Property = Property { name: "SDL_PROP_WINDOW_UIKIT_OPENGL_FRAMEBUFFER_NUMBER", value: "SDL.window.uikit.opengl.framebuffer", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_WINDOW_UIKIT_OPENGL_RENDERBUFFER_NUMBER: Property = Property { name: "SDL_PROP_WINDOW_UIKIT_OPENGL_RENDERBUFFER_NUMBER", value: "SDL.window.uikit.opengl.renderbuffer", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_WINDOW_UIKIT_OPENGL_RESOLVE_FRAMEBUFFER_NUMBER: Property = Property { name: "SDL_PROP_WINDOW_UIKIT_OPENGL_RESOLVE_FRAMEBUFFER_NUMBER", value: "SDL.window.uikit.opengl.resolve_framebuffer", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_WINDOW_KMSDRM_DEVICE_INDEX_NUMBER: Property = Property { name: "SDL_PROP_WINDOW_KMSDRM_DEVICE_INDEX_NUMBER", value: "SDL.window.kmsdrm.dev_index", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_WINDOW_KMSDRM_DRM_FD_NUMBER: Property = Property { name: "SDL_PROP_WINDOW_KMSDRM_DRM_FD_NUMBER", value: "SDL.window.kmsdrm.drm_fd", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_WINDOW_KMSDRM_GBM_DEVICE_POINTER: Property = Property { name: "SDL_PROP_WINDOW_KMSDRM_GBM_DEVICE_POINTER", value: "SDL.window.kmsdrm.gbm_dev", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_WINDOW_COCOA_WINDOW_POINTER: Property = Property { name: "SDL_PROP_WINDOW_COCOA_WINDOW_POINTER", value: "SDL.window.cocoa.window", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_WINDOW_COCOA_METAL_VIEW_TAG_NUMBER: Property = Property { name: "SDL_PROP_WINDOW_COCOA_METAL_VIEW_TAG_NUMBER", value: "SDL.window.cocoa.metal_view_tag", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_WINDOW_OPENVR_OVERLAY_ID_NUMBER: Property = Property { name: "SDL_PROP_WINDOW_OPENVR_OVERLAY_ID_NUMBER", value: "SDL.window.openvr.overlay_id", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_WINDOW_QNX_WINDOW_POINTER: Property = Property { name: "SDL_PROP_WINDOW_QNX_WINDOW_POINTER", value: "SDL.window.qnx.window", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_WINDOW_QNX_SURFACE_POINTER: Property = Property { name: "SDL_PROP_WINDOW_QNX_SURFACE_POINTER", value: "SDL.window.qnx.surface", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_WINDOW_VIVANTE_DISPLAY_POINTER: Property = Property { name: "SDL_PROP_WINDOW_VIVANTE_DISPLAY_POINTER", value: "SDL.window.vivante.display", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_WINDOW_VIVANTE_WINDOW_POINTER: Property = Property { name: "SDL_PROP_WINDOW_VIVANTE_WINDOW_POINTER", value: "SDL.window.vivante.window", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_WINDOW_VIVANTE_SURFACE_POINTER: Property = Property { name: "SDL_PROP_WINDOW_VIVANTE_SURFACE_POINTER", value: "SDL.window.vivante.surface", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_WINDOW_WIN32_HWND_POINTER: Property = Property { name: "SDL_PROP_WINDOW_WIN32_HWND_POINTER", value: "SDL.window.win32.hwnd", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_WINDOW_WIN32_HDC_POINTER: Property = Property { name: "SDL_PROP_WINDOW_WIN32_HDC_POINTER", value: "SDL.window.win32.hdc", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_WINDOW_WIN32_INSTANCE_POINTER: Property = Property { name: "SDL_PROP_WINDOW_WIN32_INSTANCE_POINTER", value: "SDL.window.win32.instance", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_WINDOW_WAYLAND_DISPLAY_POINTER: Property = Property { name: "SDL_PROP_WINDOW_WAYLAND_DISPLAY_POINTER", value: "SDL.window.wayland.display", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_WINDOW_WAYLAND_SURFACE_POINTER: Property = Property { name: "SDL_PROP_WINDOW_WAYLAND_SURFACE_POINTER", value: "SDL.window.wayland.surface", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_WINDOW_WAYLAND_VIEWPORT_POINTER: Property = Property { name: "SDL_PROP_WINDOW_WAYLAND_VIEWPORT_POINTER", value: "SDL.window.wayland.viewport", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_WINDOW_WAYLAND_EGL_WINDOW_POINTER: Property = Property { name: "SDL_PROP_WINDOW_WAYLAND_EGL_WINDOW_POINTER", value: "SDL.window.wayland.egl_window", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_WINDOW_WAYLAND_XDG_SURFACE_POINTER: Property = Property { name: "SDL_PROP_WINDOW_WAYLAND_XDG_SURFACE_POINTER", value: "SDL.window.wayland.xdg_surface", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_WINDOW_WAYLAND_XDG_TOPLEVEL_POINTER: Property = Property { name: "SDL_PROP_WINDOW_WAYLAND_XDG_TOPLEVEL_POINTER", value: "SDL.window.wayland.xdg_toplevel", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_WINDOW_WAYLAND_XDG_TOPLEVEL_EXPORT_HANDLE_STRING: Property = Property { name: "SDL_PROP_WINDOW_WAYLAND_XDG_TOPLEVEL_EXPORT_HANDLE_STRING", value: "SDL.window.wayland.xdg_toplevel_export_handle", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_WINDOW_WAYLAND_XDG_POPUP_POINTER: Property = Property { name: "SDL_PROP_WINDOW_WAYLAND_XDG_POPUP_POINTER", value: "SDL.window.wayland.xdg_popup", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_WINDOW_WAYLAND_XDG_POSITIONER_POINTER: Property = Property { name: "SDL_PROP_WINDOW_WAYLAND_XDG_POSITIONER_POINTER", value: "SDL.window.wayland.xdg_positioner", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_WINDOW_X11_DISPLAY_POINTER: Property = Property { name: "SDL_PROP_WINDOW_X11_DISPLAY_POINTER", value: "SDL.window.x11.display", ty: PropertyType::Pointer, doc: "" };
pub const SDL_PROP_WINDOW_X11_SCREEN_NUMBER: Property = Property { name: "SDL_PROP_WINDOW_X11_SCREEN_NUMBER", value: "SDL.window.x11.screen", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_WINDOW_X11_WINDOW_NUMBER: Property = Property { name: "SDL_PROP_WINDOW_X11_WINDOW_NUMBER", value: "SDL.window.x11.window", ty: PropertyType::Number, doc: "" };
pub const SDL_PROP_WINDOW_EMSCRIPTEN_CANVAS_ID_STRING: Property = Property { name: "SDL_PROP_WINDOW_EMSCRIPTEN_CANVAS_ID_STRING", value: "SDL.window.emscripten.canvas_id", ty: PropertyType::String, doc: "" };
pub const SDL_PROP_WINDOW_EMSCRIPTEN_KEYBOARD_ELEMENT_STRING: Property = Property { name: "SDL_PROP_WINDOW_EMSCRIPTEN_KEYBOARD_ELEMENT_STRING", value: "SDL.window.emscripten.keyboard_element", ty: PropertyType::String, doc: "" };
