# window | Tauri
Provides APIs to create windows, communicate with other windows and manipulate the current window.

#### [Window events](#window-events)

Events can be listened to using [Window.listen](about:/reference/javascript/api/namespacewindow/#listen):

```

import { getCurrentWindow } from "@tauri-apps/api/window";
getCurrentWindow().listen("my-window-event", ({ event, payload }) => { });
```


[References](#references)
-------------------------

### [Color](#color)

Re-exports [Color](about:/reference/javascript/api/namespacewebview/#color)

### [DragDropEvent](#dragdropevent)

Re-exports [DragDropEvent](about:/reference/javascript/api/namespacewebview/#dragdropevent)

### [LogicalPosition](#logicalposition)

Re-exports [LogicalPosition](about:/reference/javascript/api/namespacedpi/#logicalposition)

### [LogicalSize](#logicalsize)

Re-exports [LogicalSize](about:/reference/javascript/api/namespacedpi/#logicalsize)

### [PhysicalPosition](#physicalposition)

Re-exports [PhysicalPosition](about:/reference/javascript/api/namespacedpi/#physicalposition)

### [PhysicalSize](#physicalsize)

Re-exports [PhysicalSize](about:/reference/javascript/api/namespacedpi/#physicalsize)

[Enumerations](#enumerations)
-----------------------------

### [BackgroundThrottlingPolicy](#backgroundthrottlingpolicy)

Background throttling policy

#### [Since](#since)

2.0.0

#### [Enumeration Members](#enumeration-members)

##### [Disabled](#disabled)

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2051](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2051)

##### [Suspend](#suspend)

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2053](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2053)

##### [Throttle](#throttle)

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2052](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2052)

* * *

### [Effect](#effect)

Platform-specific window effects

#### [Since](#since-1)

2.0.0

#### [Enumeration Members](#enumeration-members-1)

##### [Acrylic](#acrylic)

**Windows 10/11**

#### [Notes](#notes)

This effect has bad performance when resizing/dragging the window on Windows 10 v1903+ and Windows 11 build 22000.

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2167](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2167)

##### [AppearanceBased](#appearancebased)

```

AppearanceBased: "appearanceBased";
```


A default material appropriate for the view’s effectiveAppearance. **macOS 10.14-**

###### [Deprecated](#deprecated)

since macOS 10.14. You should instead choose an appropriate semantic material.

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2067](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2067)

##### [Blur](#blur)

**Windows 7/10/11(22H1) Only**

#### [Notes](#notes-1)

This effect has bad performance when resizing/dragging the window on Windows 11 build 22621.

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2159](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2159)

##### [ContentBackground](#contentbackground)

```

ContentBackground: "contentBackground";
```


**macOS 10.14+**

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2139](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2139)

##### [Dark](#dark)

**macOS 10.14-**

###### [Deprecated](#deprecated-1)

since macOS 10.14. Use a semantic material instead.

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2079](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2079)

##### [FullScreenUI](#fullscreenui)

```

FullScreenUI: "fullScreenUI";
```


**macOS 10.14+**

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2131](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2131)

```

HeaderView: "headerView";
```


**macOS 10.14+**

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2115](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2115)

##### [HudWindow](#hudwindow)

**macOS 10.14+**

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2127](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2127)

##### [Light](#light)

**macOS 10.14-**

###### [Deprecated](#deprecated-2)

since macOS 10.14. Use a semantic material instead.

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2073](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2073)

##### [MediumLight](#mediumlight)

```

MediumLight: "mediumLight";
```


**macOS 10.14-**

###### [Deprecated](#deprecated-3)

since macOS 10.14. Use a semantic material instead.

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2085](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2085)

**macOS 10.11+**

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2103](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2103)

##### [Mica](#mica)

**Windows 11 Only**

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2151](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2151)

##### [Popover](#popover)

**macOS 10.11+**

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2107](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2107)

##### [Selection](#selection)

**macOS 10.10+**

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2099](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2099)

##### [Sheet](#sheet)

**macOS 10.14+**

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2119](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2119)

**macOS 10.11+**

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2111](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2111)

##### [Tabbed](#tabbed)

Tabbed effect that matches the system dark perefence **Windows 11 Only**

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2171](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2171)

##### [TabbedDark](#tabbeddark)

```

TabbedDark: "tabbedDark";
```


Tabbed effect with dark mode but only if dark mode is enabled on the system **Windows 11 Only**

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2175](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2175)

##### [TabbedLight](#tabbedlight)

```

TabbedLight: "tabbedLight";
```


Tabbed effect with light mode **Windows 11 Only**

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2179](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2179)

##### [Titlebar](#titlebar)

**macOS 10.10+**

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2095](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2095)

##### [Tooltip](#tooltip)

**macOS 10.14+**

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2135](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2135)

##### [UltraDark](#ultradark)

**macOS 10.14-**

###### [Deprecated](#deprecated-4)

since macOS 10.14. Use a semantic material instead.

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2091](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2091)

##### [UnderPageBackground](#underpagebackground)

```

UnderPageBackground: "underPageBackground";
```


**macOS 10.14+**

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2147](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2147)

##### [UnderWindowBackground](#underwindowbackground)

```

UnderWindowBackground: "underWindowBackground";
```


**macOS 10.14+**

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2143](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2143)

##### [WindowBackground](#windowbackground)

```

WindowBackground: "windowBackground";
```


**macOS 10.14+**

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2123](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2123)

* * *

### [EffectState](#effectstate)

Window effect state **macOS only**

#### [See](#see)

[https://developer.apple.com/documentation/appkit/nsvisualeffectview/state](https://developer.apple.com/documentation/appkit/nsvisualeffectview/state)

#### [Since](#since-2)

2.0.0

#### [Enumeration Members](#enumeration-members-2)

##### [Active](#active)

Make window effect state always active **macOS only**

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2197](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2197)

##### [FollowsWindowActiveState](#followswindowactivestate)

```

FollowsWindowActiveState: "followsWindowActiveState";
```


Make window effect state follow the window’s active state **macOS only**

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2193](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2193)

##### [Inactive](#inactive)

Make window effect state always inactive **macOS only**

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2201](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2201)

* * *

### [ProgressBarStatus](#progressbarstatus)

#### [Enumeration Members](#enumeration-members-3)

##### [Error](#error)

Error state. **Treated as Normal on linux**

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L190](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L190)

##### [Indeterminate](#indeterminate)

```

Indeterminate: "indeterminate";
```


Indeterminate state. **Treated as Normal on Linux and macOS**

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L182](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L182)

##### [None](#none)

Hide progress bar.

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L174](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L174)

##### [Normal](#normal)

Normal state.

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L178](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L178)

##### [Paused](#paused)

Paused state. **Treated as Normal on Linux**

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L186](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L186)

* * *

### [UserAttentionType](#userattentiontype)

Attention type to request on a window.

#### [Since](#since-3)

1.0.0

#### [Enumeration Members](#enumeration-members-4)

##### [Critical](#critical)

#### [Platform-specific](#platform-specific)

*   **macOS:** Bounces the dock icon until the application is in focus.
*   **Windows:** Flashes both the window and the taskbar button until the application is in focus.

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L99](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L99)

##### [Informational](#informational)

#### [Platform-specific](#platform-specific-1)

*   **macOS:** Bounces the dock icon once.
*   **Windows:** Flashes the taskbar button until the application is in focus.

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L105](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L105)

[Classes](#classes)
-------------------

### [CloseRequestedEvent](#closerequestedevent)

#### [Constructors](#constructors)

##### [new CloseRequestedEvent()](#new-closerequestedevent)

```

new CloseRequestedEvent(event): CloseRequestedEvent
```


###### [Parameters](#parameters)


|Parameter|Type          |
|---------|--------------|
|event    |Event<unknown>|


###### [Returns](#returns)

[`CloseRequestedEvent`](about:/reference/javascript/api/namespacewindow/#closerequestedevent)

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L115](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L115)

#### [Properties](#properties)



* Property:  event
  * Type: EventName
  * Description: Event name
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L110
* Property:  id
  * Type: number
  * Description: Event identifier used to unlisten
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L112


#### [Methods](#methods)

##### [isPreventDefault()](#ispreventdefault)

```

isPreventDefault(): boolean
```


###### [Returns](#returns-1)

`boolean`

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L124](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L124)

##### [preventDefault()](#preventdefault)

###### [Returns](#returns-2)

`void`

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L120](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L120)

* * *

### [Window](#window)

Create new window or get a handle to an existing one.

Windows are identified by a _label_ a unique identifier that can be used to reference it later. It may only contain alphanumeric characters `a-zA-Z` plus the following special characters `-`, `/`, `:` and `_`.

#### [Example](#example)

```

import { Window } from "@tauri-apps/api/window"
const appWindow = new Window('theUniqueLabel');
appWindow.once('tauri://created', function () {
 // window successfully created
});
appWindow.once('tauri://error', function (e) {
 // an error happened creating the window
});
// emit an event to the backend
await appWindow.emit("some-event", "data");
// listen to an event from the backend
const unlisten = await appWindow.listen("event-name", e => {});
unlisten();
```


#### [Since](#since-4)

2.0.0

#### [Extended by](#extended-by)

*   [`WebviewWindow`](about:/reference/javascript/api/namespacewebviewwindow/#webviewwindow)

#### [Constructors](#constructors-1)

##### [new Window()](#new-window)

```

new Window(label, options): Window
```


Creates a new Window.

###### [Parameters](#parameters-1)


|Parameter|Type         |Description                                               |
|---------|-------------|----------------------------------------------------------|
|label    |string       |The unique window label. Must be alphanumeric: a-zA-Z-/:_.|
|options  |WindowOptions|-                                                         |


###### [Returns](#returns-3)

[`Window`](about:/reference/javascript/api/namespacewindow/#window)

The [Window](about:/reference/javascript/api/namespacewindow/#window) instance to communicate with the window.

###### [Example](#example-1)

```

import { Window } from '@tauri-apps/api/window';
const appWindow = new Window('my-label');
appWindow.once('tauri://created', function () {
 // window successfully created
});
appWindow.once('tauri://error', function (e) {
 // an error happened creating the window
});
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L298](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L298)

#### [Properties](#properties-1)



* Property:  label
  * Type: string
  * Description: The window label. It is a unique identifier for the window, can be used to reference it later.
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L276
* Property:  listeners
  * Type: Record<string, EventCallback<any>[]>
  * Description: Local event listeners.
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L279


#### [Methods](#methods-1)

##### [center()](#center)

Centers the window.

###### [Returns](#returns-4)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-2)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
await getCurrentWindow().center();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L835](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L835)

##### [clearEffects()](#cleareffects)

```

clearEffects(): Promise<void>
```


Clear any applied effects if possible.

###### [Returns](#returns-5)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1223](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1223)

##### [close()](#close)

Closes the window.

Note this emits a closeRequested event so you can intercept it. To force window close, use [Window.destroy](about:/reference/javascript/api/namespacewindow/#destroy).

###### [Returns](#returns-6)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-3)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
await getCurrentWindow().close();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1144](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1144)

##### [destroy()](#destroy)

Destroys the window. Behaves like [Window.close](about:/reference/javascript/api/namespacewindow/#close) but forces the window close instead of emitting a closeRequested event.

###### [Returns](#returns-7)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-4)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
await getCurrentWindow().destroy();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1160](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1160)

##### [emit()](#emit)

```

emit<T>(event, payload?): Promise<void>
```


Emits an event to all [targets](about:/reference/javascript/api/namespaceevent/#eventtarget).

###### [Type Parameters](#type-parameters)


|Type Parameter|
|--------------|
|T             |


###### [Parameters](#parameters-2)


|Parameter|Type  |Description                                                          |
|---------|------|---------------------------------------------------------------------|
|event    |string|Event name. Must include only alphanumeric characters, -, /, : and _.|
|payload? |T     |Event payload.                                                       |


###### [Returns](#returns-8)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

###### [Example](#example-5)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
await getCurrentWindow().emit('window-loaded', { loggedIn: true, token: 'authToken' });
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L449](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L449)

##### [emitTo()](#emitto)

```

emitTo<T>(
   target,
   event,
payload?): Promise<void>
```


Emits an event to all [targets](about:/reference/javascript/api/namespaceevent/#eventtarget) matching the given target.

###### [Type Parameters](#type-parameters-1)


|Type Parameter|
|--------------|
|T             |


###### [Parameters](#parameters-3)



* Parameter: target
  * Type: string | EventTarget
  * Description: Label of the target Window/Webview/WebviewWindow or raw EventTarget object.
* Parameter: event
  * Type: string
  * Description: Event name. Must include only alphanumeric characters, -, /, : and _.
* Parameter: payload?
  * Type: T
  * Description: Event payload.


###### [Returns](#returns-9)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

###### [Example](#example-6)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
await getCurrentWindow().emit('main', 'window-loaded', { loggedIn: true, token: 'authToken' });
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L476](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L476)

##### [hide()](#hide)

Sets the window visibility to false.

###### [Returns](#returns-10)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-7)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
await getCurrentWindow().hide();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1126](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1126)

##### [innerPosition()](#innerposition)

```

innerPosition(): Promise<PhysicalPosition>
```


The position of the top-left hand corner of the window’s client area relative to the top-left hand corner of the desktop.

###### [Returns](#returns-11)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`PhysicalPosition`](about:/reference/javascript/api/namespacedpi/#physicalposition)\>

The window’s inner position.

###### [Example](#example-8)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
const position = await getCurrentWindow().innerPosition();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L537](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L537)

##### [innerSize()](#innersize)

```

innerSize(): Promise<PhysicalSize>
```


The physical size of the window’s client area. The client area is the content of the window, excluding the title bar and borders.

###### [Returns](#returns-12)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`PhysicalSize`](about:/reference/javascript/api/namespacedpi/#physicalsize)\>

The window’s inner size.

###### [Example](#example-9)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
const size = await getCurrentWindow().innerSize();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L570](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L570)

##### [isAlwaysOnTop()](#isalwaysontop)

```

isAlwaysOnTop(): Promise<boolean>
```


Whether the window is configured to be always on top of other windows or not.

###### [Returns](#returns-13)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`boolean`\>

Whether the window is visible or not.

###### [Example](#example-10)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
const alwaysOnTop = await getCurrentWindow().isAlwaysOnTop();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L817](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L817)

##### [isClosable()](#isclosable)

```

isClosable(): Promise<boolean>
```


Gets the window’s native close button state.

#### [Platform-specific](#platform-specific-2)

*   **iOS / Android:** Unsupported.

###### [Returns](#returns-14)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`boolean`\>

Whether the window’s native close button is enabled or not.

###### [Example](#example-11)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
const closable = await getCurrentWindow().isClosable();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L750](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L750)

##### [isDecorated()](#isdecorated)

```

isDecorated(): Promise<boolean>
```


Gets the window’s current decorated state.

###### [Returns](#returns-15)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`boolean`\>

Whether the window is decorated or not.

###### [Example](#example-12)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
const decorated = await getCurrentWindow().isDecorated();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L671](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L671)

##### [isEnabled()](#isenabled)

```

isEnabled(): Promise<boolean>
```


Whether the window is enabled or disabled.

###### [Returns](#returns-16)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`boolean`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-13)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
await getCurrentWindow().setEnabled(false);
```


###### [Since](#since-5)

2.0.0

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L927](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L927)

##### [isFocused()](#isfocused)

```

isFocused(): Promise<boolean>
```


Gets the window’s current focus state.

###### [Returns](#returns-17)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`boolean`\>

Whether the window is focused or not.

###### [Example](#example-14)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
const focused = await getCurrentWindow().isFocused();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L655](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L655)

##### [isFullscreen()](#isfullscreen)

```

isFullscreen(): Promise<boolean>
```


Gets the window’s current fullscreen state.

###### [Returns](#returns-18)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`boolean`\>

Whether the window is in fullscreen mode or not.

###### [Example](#example-15)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
const fullscreen = await getCurrentWindow().isFullscreen();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L609](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L609)

##### [isMaximizable()](#ismaximizable)

```

isMaximizable(): Promise<boolean>
```


Gets the window’s native maximize button state.

#### [Platform-specific](#platform-specific-3)

*   **Linux / iOS / Android:** Unsupported.

###### [Returns](#returns-19)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`boolean`\>

Whether the window’s native maximize button is enabled or not.

###### [Example](#example-16)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
const maximizable = await getCurrentWindow().isMaximizable();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L708](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L708)

##### [isMaximized()](#ismaximized)

```

isMaximized(): Promise<boolean>
```


Gets the window’s current maximized state.

###### [Returns](#returns-20)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`boolean`\>

Whether the window is maximized or not.

###### [Example](#example-17)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
const maximized = await getCurrentWindow().isMaximized();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L639](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L639)

##### [isMinimizable()](#isminimizable)

```

isMinimizable(): Promise<boolean>
```


Gets the window’s native minimize button state.

#### [Platform-specific](#platform-specific-4)

*   **Linux / iOS / Android:** Unsupported.

###### [Returns](#returns-21)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`boolean`\>

Whether the window’s native minimize button is enabled or not.

###### [Example](#example-18)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
const minimizable = await getCurrentWindow().isMinimizable();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L729](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L729)

##### [isMinimized()](#isminimized)

```

isMinimized(): Promise<boolean>
```


Gets the window’s current minimized state.

###### [Returns](#returns-22)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`boolean`\>

###### [Example](#example-19)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
const minimized = await getCurrentWindow().isMinimized();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L623](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L623)

##### [isResizable()](#isresizable)

```

isResizable(): Promise<boolean>
```


Gets the window’s current resizable state.

###### [Returns](#returns-23)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`boolean`\>

Whether the window is resizable or not.

###### [Example](#example-20)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
const resizable = await getCurrentWindow().isResizable();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L687](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L687)

##### [isVisible()](#isvisible)

```

isVisible(): Promise<boolean>
```


Gets the window’s current visible state.

###### [Returns](#returns-24)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`boolean`\>

Whether the window is visible or not.

###### [Example](#example-21)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
const visible = await getCurrentWindow().isVisible();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L766](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L766)

##### [listen()](#listen)

```

listen<T>(event, handler): Promise<UnlistenFn>
```


Listen to an emitted event on this window.

###### [Type Parameters](#type-parameters-2)


|Type Parameter|
|--------------|
|T             |


###### [Parameters](#parameters-4)


|Parameter|Type            |Description                                                          |
|---------|----------------|---------------------------------------------------------------------|
|event    |EventName       |Event name. Must include only alphanumeric characters, -, /, : and _.|
|handler  |EventCallback<T>|Event handler.                                                       |


###### [Returns](#returns-25)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`UnlistenFn`](about:/reference/javascript/api/namespaceevent/#unlistenfn)\>

A promise resolving to a function to unlisten to the event. Note that removing the listener is required if your listener goes out of scope e.g. the component is unmounted.

###### [Example](#example-22)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
const unlisten = await getCurrentWindow().listen<string>('state-changed', (event) => {
  console.log(`Got error: ${payload}`);
});
// you need to call unlisten if your handler goes out of scope e.g. the component is unmounted
unlisten();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L387](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L387)

##### [maximize()](#maximize)

```

maximize(): Promise<void>
```


Maximizes the window.

###### [Returns](#returns-26)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-23)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
await getCurrentWindow().maximize();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1030](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1030)

##### [minimize()](#minimize)

```

minimize(): Promise<void>
```


Minimizes the window.

###### [Returns](#returns-27)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-24)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
await getCurrentWindow().minimize();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1078](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1078)

##### [once()](#once)

```

once<T>(event, handler): Promise<UnlistenFn>
```


Listen to an emitted event on this window only once.

###### [Type Parameters](#type-parameters-3)


|Type Parameter|
|--------------|
|T             |


###### [Parameters](#parameters-5)


|Parameter|Type            |Description                                                          |
|---------|----------------|---------------------------------------------------------------------|
|event    |EventName       |Event name. Must include only alphanumeric characters, -, /, : and _.|
|handler  |EventCallback<T>|Event handler.                                                       |


###### [Returns](#returns-28)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`UnlistenFn`](about:/reference/javascript/api/namespaceevent/#unlistenfn)\>

A promise resolving to a function to unlisten to the event. Note that removing the listener is required if your listener goes out of scope e.g. the component is unmounted.

###### [Example](#example-25)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
const unlisten = await getCurrentWindow().once<null>('initialized', (event) => {
  console.log(`Window initialized!`);
});
// you need to call unlisten if your handler goes out of scope e.g. the component is unmounted
unlisten();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L422](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L422)

##### [onCloseRequested()](#oncloserequested)

```

onCloseRequested(handler): Promise<UnlistenFn>
```


Listen to window close requested. Emitted when the user requests to closes the window.

###### [Parameters](#parameters-6)


|Parameter|Type                           |
|---------|-------------------------------|
|handler  |(event) => void | Promise<void>|


###### [Returns](#returns-29)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`UnlistenFn`](about:/reference/javascript/api/namespaceevent/#unlistenfn)\>

A promise resolving to a function to unlisten to the event. Note that removing the listener is required if your listener goes out of scope e.g. the component is unmounted.

###### [Example](#example-26)

```

import { getCurrentWindow } from "@tauri-apps/api/window";
import { confirm } from '@tauri-apps/api/dialog';
const unlisten = await getCurrentWindow().onCloseRequested(async (event) => {
  const confirmed = await confirm('Are you sure?');
  if (!confirmed) {
    // user did not confirm closing the window; let's prevent it
    event.preventDefault();
  }
});
// you need to call unlisten if your handler goes out of scope e.g. the component is unmounted
unlisten();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1845](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1845)

##### [onDragDropEvent()](#ondragdropevent)

```

onDragDropEvent(handler): Promise<UnlistenFn>
```


Listen to a file drop event. The listener is triggered when the user hovers the selected files on the webview, drops the files or cancels the operation.

###### [Parameters](#parameters-7)


|Parameter|Type                        |
|---------|----------------------------|
|handler  |EventCallback<DragDropEvent>|


###### [Returns](#returns-30)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`UnlistenFn`](about:/reference/javascript/api/namespaceevent/#unlistenfn)\>

A promise resolving to a function to unlisten to the event. Note that removing the listener is required if your listener goes out of scope e.g. the component is unmounted.

###### [Example](#example-27)

```

import { getCurrentWindow } from "@tauri-apps/api/webview";
const unlisten = await getCurrentWindow().onDragDropEvent((event) => {
 if (event.payload.type === 'over') {
   console.log('User hovering', event.payload.position);
 } else if (event.payload.type === 'drop') {
   console.log('User dropped', event.payload.paths);
 } else {
   console.log('File drop cancelled');
 }
});
// you need to call unlisten if your handler goes out of scope e.g. the component is unmounted
unlisten();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1883](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1883)

##### [onFocusChanged()](#onfocuschanged)

```

onFocusChanged(handler): Promise<UnlistenFn>
```


Listen to window focus change.

###### [Parameters](#parameters-8)


|Parameter|Type                  |
|---------|----------------------|
|handler  |EventCallback<boolean>|


###### [Returns](#returns-31)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`UnlistenFn`](about:/reference/javascript/api/namespaceevent/#unlistenfn)\>

A promise resolving to a function to unlisten to the event. Note that removing the listener is required if your listener goes out of scope e.g. the component is unmounted.

###### [Example](#example-28)

```

import { getCurrentWindow } from "@tauri-apps/api/window";
const unlisten = await getCurrentWindow().onFocusChanged(({ payload: focused }) => {
 console.log('Focus changed, window is focused? ' + focused);
});
// you need to call unlisten if your handler goes out of scope e.g. the component is unmounted
unlisten();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1961](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1961)

##### [onMoved()](#onmoved)

```

onMoved(handler): Promise<UnlistenFn>
```


Listen to window move.

###### [Parameters](#parameters-9)


|Parameter|Type                           |
|---------|-------------------------------|
|handler  |EventCallback<PhysicalPosition>|


###### [Returns](#returns-32)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`UnlistenFn`](about:/reference/javascript/api/namespaceevent/#unlistenfn)\>

A promise resolving to a function to unlisten to the event. Note that removing the listener is required if your listener goes out of scope e.g. the component is unmounted.

###### [Example](#example-29)

```

import { getCurrentWindow } from "@tauri-apps/api/window";
const unlisten = await getCurrentWindow().onMoved(({ payload: position }) => {
 console.log('Window moved', position);
});
// you need to call unlisten if your handler goes out of scope e.g. the component is unmounted
unlisten();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1816](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1816)

##### [onResized()](#onresized)

```

onResized(handler): Promise<UnlistenFn>
```


Listen to window resize.

###### [Parameters](#parameters-10)


|Parameter|Type                       |
|---------|---------------------------|
|handler  |EventCallback<PhysicalSize>|


###### [Returns](#returns-33)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`UnlistenFn`](about:/reference/javascript/api/namespaceevent/#unlistenfn)\>

A promise resolving to a function to unlisten to the event. Note that removing the listener is required if your listener goes out of scope e.g. the component is unmounted.

###### [Example](#example-30)

```

import { getCurrentWindow } from "@tauri-apps/api/window";
const unlisten = await getCurrentWindow().onResized(({ payload: size }) => {
 console.log('Window resized', size);
});
// you need to call unlisten if your handler goes out of scope e.g. the component is unmounted
unlisten();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1792](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1792)

##### [onScaleChanged()](#onscalechanged)

```

onScaleChanged(handler): Promise<UnlistenFn>
```


Listen to window scale change. Emitted when the window’s scale factor has changed. The following user actions can cause DPI changes:

*   Changing the display’s resolution.
*   Changing the display’s scale factor (e.g. in Control Panel on Windows).
*   Moving the window to a display with a different scale factor.

###### [Parameters](#parameters-11)


|Parameter|Type                             |
|---------|---------------------------------|
|handler  |EventCallback<ScaleFactorChanged>|


###### [Returns](#returns-34)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`UnlistenFn`](about:/reference/javascript/api/namespaceevent/#unlistenfn)\>

A promise resolving to a function to unlisten to the event. Note that removing the listener is required if your listener goes out of scope e.g. the component is unmounted.

###### [Example](#example-31)

```

import { getCurrentWindow } from "@tauri-apps/api/window";
const unlisten = await getCurrentWindow().onScaleChanged(({ payload }) => {
 console.log('Scale changed', payload.scaleFactor, payload.size);
});
// you need to call unlisten if your handler goes out of scope e.g. the component is unmounted
unlisten();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2001](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2001)

##### [onThemeChanged()](#onthemechanged)

```

onThemeChanged(handler): Promise<UnlistenFn>
```


Listen to the system theme change.

###### [Parameters](#parameters-12)


|Parameter|Type                |
|---------|--------------------|
|handler  |EventCallback<Theme>|


###### [Returns](#returns-35)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`UnlistenFn`](about:/reference/javascript/api/namespaceevent/#unlistenfn)\>

A promise resolving to a function to unlisten to the event. Note that removing the listener is required if your listener goes out of scope e.g. the component is unmounted.

###### [Example](#example-32)

```

import { getCurrentWindow } from "@tauri-apps/api/window";
const unlisten = await getCurrentWindow().onThemeChanged(({ payload: theme }) => {
 console.log('New theme: ' + theme);
});
// you need to call unlisten if your handler goes out of scope e.g. the component is unmounted
unlisten();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2027](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2027)

##### [outerPosition()](#outerposition)

```

outerPosition(): Promise<PhysicalPosition>
```


The position of the top-left hand corner of the window relative to the top-left hand corner of the desktop.

###### [Returns](#returns-36)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`PhysicalPosition`](about:/reference/javascript/api/namespacedpi/#physicalposition)\>

The window’s outer position.

###### [Example](#example-33)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
const position = await getCurrentWindow().outerPosition();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L553](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L553)

##### [outerSize()](#outersize)

```

outerSize(): Promise<PhysicalSize>
```


The physical size of the entire window. These dimensions include the title bar and borders. If you don’t want that (and you usually don’t), use inner\_size instead.

###### [Returns](#returns-37)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`PhysicalSize`](about:/reference/javascript/api/namespacedpi/#physicalsize)\>

The window’s outer size.

###### [Example](#example-34)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
const size = await getCurrentWindow().outerSize();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L590](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L590)

##### [requestUserAttention()](#requestuserattention)

```

requestUserAttention(requestType): Promise<void>
```


Requests user attention to the window, this has no effect if the application is already focused. How requesting for user attention manifests is platform dependent, see `UserAttentionType` for details.

Providing `null` will unset the request for user attention. Unsetting the request for user attention might not be done automatically by the WM when the window receives input.

#### [Platform-specific](#platform-specific-5)

*   **macOS:** `null` has no effect.
*   **Linux:** Urgency levels have the same effect.

###### [Parameters](#parameters-13)


|Parameter  |Type                    |
|-----------|------------------------|
|requestType|null | UserAttentionType|


###### [Returns](#returns-38)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-35)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
await getCurrentWindow().requestUserAttention();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L861](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L861)

##### [scaleFactor()](#scalefactor)

```

scaleFactor(): Promise<number>
```


The scale factor that can be used to map physical pixels to logical pixels.

###### [Returns](#returns-39)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`number`\>

The window’s monitor scale factor.

###### [Example](#example-36)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
const factor = await getCurrentWindow().scaleFactor();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L521](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L521)

##### [setAlwaysOnBottom()](#setalwaysonbottom)

```

setAlwaysOnBottom(alwaysOnBottom): Promise<void>
```


Whether the window should always be below other windows.

###### [Parameters](#parameters-14)


|Parameter     |Type   |Description                                                    |
|--------------|-------|---------------------------------------------------------------|
|alwaysOnBottom|boolean|Whether the window should always be below other windows or not.|


###### [Returns](#returns-40)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-37)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
await getCurrentWindow().setAlwaysOnBottom(true);
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1259](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1259)

##### [setAlwaysOnTop()](#setalwaysontop)

```

setAlwaysOnTop(alwaysOnTop): Promise<void>
```


Whether the window should always be on top of other windows.

###### [Parameters](#parameters-15)


|Parameter  |Type   |Description                                                        |
|-----------|-------|-------------------------------------------------------------------|
|alwaysOnTop|boolean|Whether the window should always be on top of other windows or not.|


###### [Returns](#returns-41)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-38)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
await getCurrentWindow().setAlwaysOnTop(true);
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1241](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1241)

##### [setBackgroundColor()](#setbackgroundcolor)

```

setBackgroundColor(color): Promise<void>
```


Sets the window background color.

#### [Platform-specific:](#platform-specific-6)

*   **Windows:** alpha channel is ignored.
*   **iOS / Android:** Unsupported.

###### [Parameters](#parameters-16)


|Parameter|Type |
|---------|-----|
|color    |Color|


###### [Returns](#returns-42)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Since](#since-6)

2.1.0

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1553](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1553)

##### [setBadgeCount()](#setbadgecount)

```

setBadgeCount(count?): Promise<void>
```


Sets the badge count. It is app wide and not specific to this window.

#### [Platform-specific](#platform-specific-7)

*   **Windows**: Unsupported. Use @{linkcode Window.setOverlayIcon} instead.

###### [Parameters](#parameters-17)


|Parameter|Type  |Description                                        |
|---------|------|---------------------------------------------------|
|count?   |number|The badge count. Use undefined to remove the badge.|


###### [Returns](#returns-43)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-39)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
await getCurrentWindow().setBadgeCount(5);
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1645](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1645)

##### [setBadgeLabel()](#setbadgelabel)

```

setBadgeLabel(label?): Promise<void>
```


Sets the badge cont **macOS only**.

###### [Parameters](#parameters-18)


|Parameter|Type  |Description                                        |
|---------|------|---------------------------------------------------|
|label?   |string|The badge label. Use undefined to remove the badge.|


###### [Returns](#returns-44)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-40)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
await getCurrentWindow().setBadgeLabel("Hello");
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1664](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1664)

##### [setClosable()](#setclosable)

```

setClosable(closable): Promise<void>
```


Sets whether the window’s native close button is enabled or not.

#### [Platform-specific](#platform-specific-8)

*   **Linux:** GTK+ will do its best to convince the window manager not to show a close button. Depending on the system, this function may not have any effect when called on a window that is already visible
*   **iOS / Android:** Unsupported.

###### [Parameters](#parameters-19)


|Parameter|Type   |
|---------|-------|
|closable |boolean|


###### [Returns](#returns-45)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-41)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
await getCurrentWindow().setClosable(false);
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L995](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L995)

##### [setContentProtected()](#setcontentprotected)

```

setContentProtected(protected_): Promise<void>
```


Prevents the window contents from being captured by other apps.

###### [Parameters](#parameters-20)


|Parameter |Type   |
|----------|-------|
|protected_|boolean|


###### [Returns](#returns-46)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-42)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
await getCurrentWindow().setContentProtected(true);
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1276](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1276)

##### [setCursorGrab()](#setcursorgrab)

```

setCursorGrab(grab): Promise<void>
```


Grabs the cursor, preventing it from leaving the window.

There’s no guarantee that the cursor will be hidden. You should hide it by yourself if you want so.

#### [Platform-specific](#platform-specific-9)

*   **Linux:** Unsupported.
*   **macOS:** This locks the cursor in a fixed location, which looks visually awkward.

###### [Parameters](#parameters-21)


|Parameter|Type   |Description                                       |
|---------|-------|--------------------------------------------------|
|grab     |boolean|true to grab the cursor icon, false to release it.|


###### [Returns](#returns-47)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-43)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
await getCurrentWindow().setCursorGrab(true);
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1492](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1492)

##### [setCursorIcon()](#setcursoricon)

```

setCursorIcon(icon): Promise<void>
```


Modifies the cursor icon of the window.

###### [Parameters](#parameters-22)


|Parameter|Type      |Description         |
|---------|----------|--------------------|
|icon     |CursorIcon|The new cursor icon.|


###### [Returns](#returns-48)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-44)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
await getCurrentWindow().setCursorIcon('help');
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1534](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1534)

##### [setCursorPosition()](#setcursorposition)

```

setCursorPosition(position): Promise<void>
```


Changes the position of the cursor in window coordinates.

###### [Parameters](#parameters-23)


|Parameter|Type                                         |Description             |
|---------|---------------------------------------------|------------------------|
|position |LogicalPosition | PhysicalPosition | Position|The new cursor position.|


###### [Returns](#returns-49)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-45)

```

import { getCurrentWindow, LogicalPosition } from '@tauri-apps/api/window';
await getCurrentWindow().setCursorPosition(new LogicalPosition(600, 300));
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1568](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1568)

##### [setCursorVisible()](#setcursorvisible)

```

setCursorVisible(visible): Promise<void>
```


Modifies the cursor’s visibility.

#### [Platform-specific](#platform-specific-10)

*   **Windows:** The cursor is only hidden within the confines of the window.
*   **macOS:** The cursor is hidden as long as the window has input focus, even if the cursor is outside of the window.

###### [Parameters](#parameters-24)


|Parameter|Type   |Description                                                             |
|---------|-------|------------------------------------------------------------------------|
|visible  |boolean|If false, this will hide the cursor. If true, this will show the cursor.|


###### [Returns](#returns-50)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-46)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
await getCurrentWindow().setCursorVisible(false);
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1516](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1516)

##### [setDecorations()](#setdecorations)

```

setDecorations(decorations): Promise<void>
```


Whether the window should have borders and bars.

###### [Parameters](#parameters-25)


|Parameter  |Type   |Description                                     |
|-----------|-------|------------------------------------------------|
|decorations|boolean|Whether the window should have borders and bars.|


###### [Returns](#returns-51)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-47)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
await getCurrentWindow().setDecorations(false);
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1177](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1177)

##### [setEffects()](#seteffects)

```

setEffects(effects): Promise<void>
```


Set window effects.

###### [Parameters](#parameters-26)


|Parameter|Type   |
|---------|-------|
|effects  |Effects|


###### [Returns](#returns-52)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1213](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1213)

##### [setEnabled()](#setenabled)

```

setEnabled(enabled): Promise<void>
```


Enable or disable the window.

###### [Parameters](#parameters-27)


|Parameter|Type   |
|---------|-------|
|enabled  |boolean|


###### [Returns](#returns-53)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-48)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
await getCurrentWindow().setEnabled(false);
```


###### [Since](#since-7)

2.0.0

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L908](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L908)

##### [setFocus()](#setfocus)

```

setFocus(): Promise<void>
```


Bring the window to front and focus.

###### [Returns](#returns-54)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-49)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
await getCurrentWindow().setFocus();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1418](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1418)

##### [setFullscreen()](#setfullscreen)

```

setFullscreen(fullscreen): Promise<void>
```


Sets the window fullscreen state.

###### [Parameters](#parameters-28)


|Parameter |Type   |Description                                       |
|----------|-------|--------------------------------------------------|
|fullscreen|boolean|Whether the window should go to fullscreen or not.|


###### [Returns](#returns-55)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-50)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
await getCurrentWindow().setFullscreen(true);
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1401](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1401)

##### [setIcon()](#seticon)

```

setIcon(icon): Promise<void>
```


Sets the window icon.

###### [Parameters](#parameters-29)



* Parameter: icon
  * Type: | string | number[] | ArrayBuffer | Uint8Array<ArrayBufferLike> | Image
  * Description: Icon bytes or path to the icon file.


###### [Returns](#returns-56)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-51)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
await getCurrentWindow().setIcon('/tauri/awesome.png');
```


Note that you may need the `image-ico` or `image-png` Cargo features to use this API. To enable it, change your Cargo.toml file:

```

[dependencies]
tauri = { version = "...", features = ["...", "image-png"] }
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1442](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1442)

##### [setIgnoreCursorEvents()](#setignorecursorevents)

```

setIgnoreCursorEvents(ignore): Promise<void>
```


Changes the cursor events behavior.

###### [Parameters](#parameters-30)


|Parameter|Type   |Description                                                      |
|---------|-------|-----------------------------------------------------------------|
|ignore   |boolean|true to ignore the cursor events; false to process them as usual.|


###### [Returns](#returns-57)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-52)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
await getCurrentWindow().setIgnoreCursorEvents(true);
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1589](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1589)

##### [setMaximizable()](#setmaximizable)

```

setMaximizable(maximizable): Promise<void>
```


Sets whether the window’s native maximize button is enabled or not. If resizable is set to false, this setting is ignored.

#### [Platform-specific](#platform-specific-11)

*   **macOS:** Disables the “zoom” button in the window titlebar, which is also used to enter fullscreen mode.
*   **Linux / iOS / Android:** Unsupported.

###### [Parameters](#parameters-31)


|Parameter  |Type   |
|-----------|-------|
|maximizable|boolean|


###### [Returns](#returns-58)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-53)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
await getCurrentWindow().setMaximizable(false);
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L950](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L950)

##### [setMaxSize()](#setmaxsize)

```

setMaxSize(size): Promise<void>
```


Sets the window maximum inner size. If the `size` argument is undefined, the constraint is unset.

###### [Parameters](#parameters-32)



* Parameter: size
  * Type: | undefined | null | LogicalSize | PhysicalSize | Size
  * Description: The logical or physical inner size, or null to unset the constraint.


###### [Returns](#returns-59)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-54)

```

import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window';
await getCurrentWindow().setMaxSize(new LogicalSize(600, 500));
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1332](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1332)

##### [setMinimizable()](#setminimizable)

```

setMinimizable(minimizable): Promise<void>
```


Sets whether the window’s native minimize button is enabled or not.

#### [Platform-specific](#platform-specific-12)

*   **Linux / iOS / Android:** Unsupported.

###### [Parameters](#parameters-33)


|Parameter  |Type   |
|-----------|-------|
|minimizable|boolean|


###### [Returns](#returns-60)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-55)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
await getCurrentWindow().setMinimizable(false);
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L972](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L972)

##### [setMinSize()](#setminsize)

```

setMinSize(size): Promise<void>
```


Sets the window minimum inner size. If the `size` argument is not provided, the constraint is unset.

###### [Parameters](#parameters-34)



* Parameter: size
  * Type: | undefined | null | LogicalSize | PhysicalSize | Size
  * Description: The logical or physical inner size, or null to unset the constraint.


###### [Returns](#returns-61)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-56)

```

import { getCurrentWindow, PhysicalSize } from '@tauri-apps/api/window';
await getCurrentWindow().setMinSize(new PhysicalSize(600, 500));
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1312](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1312)

##### [setOverlayIcon()](#setoverlayicon)

```

setOverlayIcon(icon?): Promise<void>
```


Sets the overlay icon. **Windows only** The overlay icon can be set for every window.

Note that you may need the `image-ico` or `image-png` Cargo features to use this API. To enable it, change your Cargo.toml file:

```

[dependencies]
tauri = { version = "...", features = ["...", "image-png"] }
```


###### [Parameters](#parameters-35)



* Parameter: icon?
  * Type: | string | number[] | ArrayBuffer | Uint8Array<ArrayBufferLike> | Image
  * Description: Icon bytes or path to the icon file. Use undefined to remove the overlay icon.


###### [Returns](#returns-62)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-57)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
await getCurrentWindow().setOverlayIcon("/tauri/awesome.png");
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1693](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1693)

##### [setPosition()](#setposition)

```

setPosition(position): Promise<void>
```


Sets the window outer position.

###### [Parameters](#parameters-36)



* Parameter: position
  * Type: LogicalPosition | PhysicalPosition | Position
  * Description: The new position, in logical or physical pixels.


###### [Returns](#returns-63)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-58)

```

import { getCurrentWindow, LogicalPosition } from '@tauri-apps/api/window';
await getCurrentWindow().setPosition(new LogicalPosition(600, 500));
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1381](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1381)

##### [setProgressBar()](#setprogressbar)

```

setProgressBar(state): Promise<void>
```


Sets the taskbar progress state.

#### [Platform-specific](#platform-specific-13)

*   **Linux / macOS**: Progress bar is app-wide and not specific to this window.
*   **Linux**: Only supported desktop environments with `libunity` (e.g. GNOME).

###### [Parameters](#parameters-37)


|Parameter|Type            |
|---------|----------------|
|state    |ProgressBarState|


###### [Returns](#returns-64)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-59)

```

import { getCurrentWindow, ProgressBarStatus } from '@tauri-apps/api/window';
await getCurrentWindow().setProgressBar({
  status: ProgressBarStatus.Normal,
  progress: 50,
});
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1721](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1721)

##### [setResizable()](#setresizable)

```

setResizable(resizable): Promise<void>
```


Updates the window resizable flag.

###### [Parameters](#parameters-38)


|Parameter|Type   |
|---------|-------|
|resizable|boolean|


###### [Returns](#returns-65)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-60)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
await getCurrentWindow().setResizable(false);
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L889](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L889)

##### [setShadow()](#setshadow)

```

setShadow(enable): Promise<void>
```


Whether or not the window should have shadow.

#### [Platform-specific](#platform-specific-14)

*   **Windows:**
    *   `false` has no effect on decorated window, shadows are always ON.
    *   `true` will make undecorated window have a 1px white border, and on Windows 11, it will have a rounded corners.
*   **Linux:** Unsupported.

###### [Parameters](#parameters-39)


|Parameter|Type   |
|---------|-------|
|enable   |boolean|


###### [Returns](#returns-66)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-61)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
await getCurrentWindow().setShadow(false);
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1203](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1203)

##### [setSize()](#setsize)

```

setSize(size): Promise<void>
```


Resizes the window with a new inner size.

###### [Parameters](#parameters-40)


|Parameter|Type                             |Description                        |
|---------|---------------------------------|-----------------------------------|
|size     |LogicalSize | PhysicalSize | Size|The logical or physical inner size.|


###### [Returns](#returns-67)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-62)

```

import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window';
await getCurrentWindow().setSize(new LogicalSize(600, 500));
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1294](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1294)

##### [setSizeConstraints()](#setsizeconstraints)

```

setSizeConstraints(constraints): Promise<void>
```


Sets the window inner size constraints.

###### [Parameters](#parameters-41)



* Parameter: constraints
  * Type: undefined | null | WindowSizeConstraints
  * Description: The logical or physical inner size, or null to unset the constraint.


###### [Returns](#returns-68)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-63)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
await getCurrentWindow().setSizeConstraints({ minWidth: 300 });
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1352](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1352)

##### [setSkipTaskbar()](#setskiptaskbar)

```

setSkipTaskbar(skip): Promise<void>
```


Whether the window icon should be hidden from the taskbar or not.

#### [Platform-specific](#platform-specific-15)

*   **macOS:** Unsupported.

###### [Parameters](#parameters-42)


|Parameter|Type   |Description                                |
|---------|-------|-------------------------------------------|
|skip     |boolean|true to hide window icon, false to show it.|


###### [Returns](#returns-69)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-64)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
await getCurrentWindow().setSkipTaskbar(true);
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1466](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1466)

##### [setTheme()](#settheme)

```

setTheme(theme?): Promise<void>
```


Set window theme, pass in `null` or `undefined` to follow system theme

#### [Platform-specific](#platform-specific-16)

*   **Linux / macOS**: Theme is app-wide and not specific to this window.
*   **iOS / Android:** Unsupported.

###### [Parameters](#parameters-43)


|Parameter|Type        |
|---------|------------|
|theme?   |null | Theme|


###### [Returns](#returns-70)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

###### [Since](#since-8)

2.0.0

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1766](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1766)

##### [setTitle()](#settitle)

```

setTitle(title): Promise<void>
```


Sets the window title.

###### [Parameters](#parameters-44)


|Parameter|Type  |Description  |
|---------|------|-------------|
|title    |string|The new title|


###### [Returns](#returns-71)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-65)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
await getCurrentWindow().setTitle('Tauri');
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1013](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1013)

##### [setTitleBarStyle()](#settitlebarstyle)

```

setTitleBarStyle(style): Promise<void>
```


Sets the title bar style. **macOS only**.

###### [Parameters](#parameters-45)


|Parameter|Type         |
|---------|-------------|
|style    |TitleBarStyle|


###### [Returns](#returns-72)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

###### [Since](#since-9)

2.0.0

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1749](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1749)

##### [setVisibleOnAllWorkspaces()](#setvisibleonallworkspaces)

```

setVisibleOnAllWorkspaces(visible): Promise<void>
```


Sets whether the window should be visible on all workspaces or virtual desktops.

#### [Platform-specific](#platform-specific-17)

*   **Windows / iOS / Android:** Unsupported.

###### [Parameters](#parameters-46)


|Parameter|Type   |
|---------|-------|
|visible  |boolean|


###### [Returns](#returns-73)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

###### [Since](#since-10)

2.0.0

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1737](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1737)

##### [show()](#show)

Sets the window visibility to true.

###### [Returns](#returns-74)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-66)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
await getCurrentWindow().show();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1110](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1110)

##### [startDragging()](#startdragging)

```

startDragging(): Promise<void>
```


Starts dragging the window.

###### [Returns](#returns-75)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-67)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
await getCurrentWindow().startDragging();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1606](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1606)

##### [startResizeDragging()](#startresizedragging)

```

startResizeDragging(direction): Promise<void>
```


Starts resize-dragging the window.

###### [Parameters](#parameters-47)


|Parameter|Type           |
|---------|---------------|
|direction|ResizeDirection|


###### [Returns](#returns-76)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-68)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
await getCurrentWindow().startResizeDragging();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1622](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1622)

##### [theme()](#theme)

```

theme(): Promise<null | Theme>
```


Gets the window’s current theme.

#### [Platform-specific](#platform-specific-18)

*   **macOS:** Theme was introduced on macOS 10.14. Returns `light` on macOS 10.13 and below.

###### [Returns](#returns-77)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`null` | [`Theme`](about:/reference/javascript/api/namespacewindow/#theme-2)\>

The window theme.

###### [Example](#example-69)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
const theme = await getCurrentWindow().theme();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L801](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L801)

##### [title()](#title)

Gets the window’s current title.

###### [Returns](#returns-78)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`string`\>

###### [Example](#example-70)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
const title = await getCurrentWindow().title();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L780](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L780)

##### [toggleMaximize()](#togglemaximize)

```

toggleMaximize(): Promise<void>
```


Toggles the window maximized state.

###### [Returns](#returns-79)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-71)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
await getCurrentWindow().toggleMaximize();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1062](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1062)

##### [unmaximize()](#unmaximize)

```

unmaximize(): Promise<void>
```


Unmaximizes the window.

###### [Returns](#returns-80)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-72)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
await getCurrentWindow().unmaximize();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1046](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1046)

##### [unminimize()](#unminimize)

```

unminimize(): Promise<void>
```


Unminimizes the window.

###### [Returns](#returns-81)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`void`\>

A promise indicating the success or failure of the operation.

###### [Example](#example-73)

```

import { getCurrentWindow } from '@tauri-apps/api/window';
await getCurrentWindow().unminimize();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1094](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L1094)

##### [getAll()](#getall)

```

static getAll(): Promise<Window[]>
```


Gets a list of instances of `Window` for all available windows.

###### [Returns](#returns-82)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`Window`](about:/reference/javascript/api/namespacewindow/#window)\[\]>

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L345](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L345)

##### [getByLabel()](#getbylabel)

```

static getByLabel(label): Promise<null | Window>
```


Gets the Window associated with the given label.

###### [Parameters](#parameters-48)


|Parameter|Type  |Description      |
|---------|------|-----------------|
|label    |string|The window label.|


###### [Returns](#returns-83)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`null` | [`Window`](about:/reference/javascript/api/namespacewindow/#window)\>

The Window instance to communicate with the window or null if the window doesn’t exist.

###### [Example](#example-74)

```

import { Window } from '@tauri-apps/api/window';
const mainWindow = Window.getByLabel('main');
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L331](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L331)

##### [getCurrent()](#getcurrent)

```

static getCurrent(): Window
```


Get an instance of `Window` for the current window.

###### [Returns](#returns-84)

[`Window`](about:/reference/javascript/api/namespacewindow/#window)

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L338](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L338)

##### [getFocusedWindow()](#getfocusedwindow)

```

static getFocusedWindow(): Promise<null | Window>
```


Gets the focused window.

###### [Returns](#returns-85)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<`null` | [`Window`](about:/reference/javascript/api/namespacewindow/#window)\>

The Window instance or `undefined` if there is not any focused window.

###### [Example](#example-75)

```

import { Window } from '@tauri-apps/api/window';
const focusedWindow = Window.getFocusedWindow();
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L359](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L359)

[Interfaces](#interfaces)
-------------------------

### [Effects](#effects)

The window effects configuration object

#### [Since](#since-11)

2.0.0

#### [Properties](#properties-2)



* Property:  color?
  * Type: Color
  * Description: Window effect color. Affects Effect.Blur and Effect.Acrylic only on Windows 10 v1903+. Doesn’t have any effect on Windows 7 or Windows 11.
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2226
* Property:  effects
  * Type: Effect[]
  * Description: List of Window effects to apply to the Window. Conflicting effects will apply the first one and ignore the rest.
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2213
* Property:  radius?
  * Type: number
  * Description: Window effect corner radius macOS Only
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2221
* Property:  state?
  * Type: EffectState
  * Description: Window effect state macOS Only
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2217


* * *

### [Monitor](#monitor)

Allows you to retrieve information about a given monitor.

#### [Since](#since-12)

1.0.0

#### [Properties](#properties-3)



* Property:  name
  * Type: null | string
  * Description: Human-readable name of the monitor
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L49
* Property:  position
  * Type: PhysicalPosition
  * Description: the Top-left corner position of the monitor relative to the larger full screen area.
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L53
* Property:  scaleFactor
  * Type: number
  * Description: The scale factor that can be used to map physical pixels to logical pixels.
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L60
* Property:  size
  * Type: PhysicalSize
  * Description: The monitor’s resolution.
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L51
* Property:  workArea
  * Type: object
  * Description: The monitor’s work area.
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L55
* Property:  workArea.position
  * Type: PhysicalPosition
  * Description: -
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L56
* Property:  workArea.size
  * Type: PhysicalSize
  * Description: -
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L57


* * *

### [ProgressBarState](#progressbarstate)

#### [Properties](#properties-4)



* Property:  progress?
  * Type: number
  * Description: The progress bar progress. This can be a value ranging from 0 to 100
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L208
* Property:  status?
  * Type: ProgressBarStatus
  * Description: The progress bar status.
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L204


* * *

### [ScaleFactorChanged](#scalefactorchanged)

The payload for the `scaleChange` event.

#### [Since](#since-13)

1.0.2

#### [Properties](#properties-5)



* Property:  scaleFactor
  * Type: number
  * Description: The new window scale factor.
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L83
* Property:  size
  * Type: PhysicalSize
  * Description: The new window size
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L85


* * *

### [WindowOptions](#windowoptions)

Configuration for the window to create.

#### [Since](#since-14)

1.0.0

#### [Properties](#properties-6)



* Property:  allowLinkPreview?
  * Type: boolean
  * Description: on macOS and iOS there is a link preview on long pressing links, this is enabled by default. see https://docs.rs/objc2-web-kit/latest/objc2_web_kit/struct.WKWebView.html#method.allowsLinkPreview
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2418
* Property:  alwaysOnBottom?
  * Type: boolean
  * Description: Whether the window should always be below other windows.
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2299
* Property:  alwaysOnTop?
  * Type: boolean
  * Description: Whether the window should always be on top of other windows or not.
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2297
* Property:  backgroundColor?
  * Type: Color
  * Description: Set the window background color. #### Platform-specific: - Android / iOS: Unsupported. - Windows: alpha channel is ignored. Since 2.1.0
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2395
* Property:  backgroundThrottling?
  * Type: BackgroundThrottlingPolicy
  * Description: Change the default background throttling behaviour. ## Platform-specific - Linux / Windows / Android: Unsupported. Workarounds like a pending WebLock transaction might suffice. - iOS: Supported since version 17.0+. - macOS: Supported since version 14.0+. see https://github.com/tauri-apps/tauri/issues/5250#issuecomment-2569380578 Since 2.3.0
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2409
* Property:  center?
  * Type: boolean
  * Description: Show window in the center of the screen..
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2244
* Property:  closable?
  * Type: boolean
  * Description: Whether the window’s native close button is enabled or not. Defaults to true.
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2350
* Property:  contentProtected?
  * Type: boolean
  * Description: Prevents the window contents from being captured by other apps.
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2301
* Property:  decorations?
  * Type: boolean
  * Description: Whether the window should have borders and bars or not.
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2295
* Property:  disableInputAccessoryView?
  * Type: boolean
  * Description: Allows disabling the input accessory view on iOS. The accessory view is the view that appears above the keyboard when a text input element is focused. It usually displays a view with “Done”, “Next” buttons.
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2425
* Property:  focus?
  * Type: boolean
  * Description: Whether the window will be initially focused or not.
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2283
* Property:  fullscreen?
  * Type: boolean
  * Description: Whether the window is in fullscreen mode or not.
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2281
* Property:  height?
  * Type: number
  * Description: The initial height.
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2252
* Property:  hiddenTitle?
  * Type: boolean
  * Description: If true, sets the window title to be hidden on macOS.
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2331
* Property:  javascriptDisabled?
  * Type: boolean
  * Description: Whether we should disable JavaScript code execution on the webview or not.
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2413
* Property:  maxHeight?
  * Type: number
  * Description: The maximum height. Only applies if maxWidth is also set.
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2260
* Property:  maximizable?
  * Type: boolean
  * Description: Whether the window’s native maximize button is enabled or not. Defaults to true.
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2342
* Property:  maximized?
  * Type: boolean
  * Description: Whether the window should be maximized upon creation or not.
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2291
* Property:  maxWidth?
  * Type: number
  * Description: The maximum width. Only applies if maxHeight is also set.
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2258
* Property:  minHeight?
  * Type: number
  * Description: The minimum height. Only applies if minWidth is also set.
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2256
* Property:  minimizable?
  * Type: boolean
  * Description: Whether the window’s native minimize button is enabled or not. Defaults to true.
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2346
* Property:  minWidth?
  * Type: number
  * Description: The minimum width. Only applies if minHeight is also set.
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2254
* Property:  parent?
  * Type: string | Window | WebviewWindow
  * Description: Sets a parent to the window to be created. Can be either a Window or a label of the window. #### Platform-specific - Windows: This sets the passed parent as an owner window to the window to be created. From MSDN owned windows docs: - An owned window is always above its owner in the z-order. - The system automatically destroys an owned window when its owner is destroyed. - An owned window is hidden when its owner is minimized. - Linux: This makes the new window transient for parent, see https://docs.gtk.org/gtk3/method.Window.set_transient_for.html - macOS: This adds the window as a child of parent, see https://developer.apple.com/documentation/appkit/nswindow/1419152-addchildwindow?language=objc
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2364
* Property:  preventOverflow?
  * Type: boolean | PreventOverflowMargin
  * Description: Prevent the window from overflowing the working area (e.g. monitor size - taskbar size) on creation, which means the window size will be limited to monitor size - taskbar size Can either be set to true or to a PreventOverflowMargin object to set an additional margin that should be considered to determine the working area (in this case the window size will be limited to monitor size - taskbar size - margin) NOTE: The overflow check is only performed on window creation, resizes can still overflow #### Platform-specific - iOS / Android: Unsupported.
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2275
* Property:  resizable?
  * Type: boolean
  * Description: Whether the window is resizable or not.
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2277
* Property:  shadow?
  * Type: boolean
  * Description: Whether or not the window has shadow. #### Platform-specific - Windows: - false has no effect on decorated window, shadows are always ON. - true will make undecorated window have a 1px white border, and on Windows 11, it will have a rounded corners. - Linux: Unsupported. Since 2.0.0
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2317
* Property:  skipTaskbar?
  * Type: boolean
  * Description: Whether or not the window icon should be added to the taskbar.
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2303
* Property:  tabbingIdentifier?
  * Type: string
  * Description: Defines the window tabbing identifier on macOS. Windows with the same tabbing identifier will be grouped together. If the tabbing identifier is not set, automatic tabbing will be disabled.
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2338
* Property:  theme?
  * Type: Theme
  * Description: The initial window theme. Defaults to the system theme. Only implemented on Windows and macOS 10.14+.
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2323
* Property:  title?
  * Type: string
  * Description: Window title.
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2279
* Property:  titleBarStyle?
  * Type: TitleBarStyle
  * Description: The style of the macOS title bar.
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2327
* Property:  transparent?
  * Type: boolean
  * Description: Whether the window is transparent or not. Note that on macOS this requires the macos-private-api feature flag, enabled under tauri.conf.json > app > macOSPrivateApi. WARNING: Using private APIs on macOS prevents your application from being accepted to the App Store.
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2289
* Property:  visible?
  * Type: boolean
  * Description: Whether the window should be immediately visible upon creation or not.
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2293
* Property:  visibleOnAllWorkspaces?
  * Type: boolean
  * Description: Whether the window should be visible on all workspaces or virtual desktops. #### Platform-specific - Windows / iOS / Android: Unsupported. Since 2.0.0
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2373
* Property:  width?
  * Type: number
  * Description: The initial width.
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2250
* Property:  windowEffects?
  * Type: Effects
  * Description: Window effects. Requires the window to be transparent. #### Platform-specific: - Windows: If using decorations or shadows, you may want to try this workaround https://github.com/tauri-apps/tao/issues/72#issuecomment-975607891 - Linux: Unsupported
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2384
* Property:  x?
  * Type: number
  * Description: The initial vertical position. Only applies if y is also set.
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2246
* Property:  y?
  * Type: number
  * Description: The initial horizontal position. Only applies if x is also set.
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2248


* * *

### [WindowSizeConstraints](#windowsizeconstraints)

#### [Properties](#properties-7)



* Property:  maxHeight?
  * Type: number
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L197
* Property:  maxWidth?
  * Type: number
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L196
* Property:  minHeight?
  * Type: number
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L195
* Property:  minWidth?
  * Type: number
  * Defined in: Source: https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L194


[Type Aliases](#type-aliases)
-----------------------------

### [CursorIcon](#cursoricon)

```

type CursorIcon:
  | "default"
  | "crosshair"
  | "hand"
  | "arrow"
  | "move"
  | "text"
  | "wait"
  | "help"
  | "progress"
  | "notAllowed"
  | "contextMenu"
  | "cell"
  | "verticalText"
  | "alias"
  | "copy"
  | "noDrop"
  | "grab"
  | "grabbing"
  | "allScroll"
  | "zoomIn"
  | "zoomOut"
  | "eResize"
  | "nResize"
  | "neResize"
  | "nwResize"
  | "sResize"
  | "seResize"
  | "swResize"
  | "wResize"
  | "ewResize"
  | "nsResize"
  | "neswResize"
  | "nwseResize"
  | "colResize"
  | "rowResize";
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L129](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L129)

* * *

### [Theme](#theme-1)

```

type Theme: "light" | "dark";
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L63](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L63)

* * *

### [TitleBarStyle](#titlebarstyle)

```

type TitleBarStyle: "visible" | "transparent" | "overlay";
```


**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L64](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L64)

[Functions](#functions)
-----------------------

### [availableMonitors()](#availablemonitors)

```

function availableMonitors(): Promise<Monitor[]>
```


Returns the list of all the monitors available on the system.

#### [Returns](#returns-86)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`Monitor`](about:/reference/javascript/api/namespacewindow/#monitor)\[\]>

#### [Example](#example-76)

```

import { availableMonitors } from '@tauri-apps/api/window';
const monitors = await availableMonitors();
```


#### [Since](#since-15)

1.0.0

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2504](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2504)

* * *

### [currentMonitor()](#currentmonitor)

```

function currentMonitor(): Promise<Monitor | null>
```


Returns the monitor on which the window currently resides. Returns `null` if current monitor can’t be detected.

#### [Returns](#returns-87)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`Monitor`](about:/reference/javascript/api/namespacewindow/#monitor) | `null`\>

#### [Example](#example-77)

```

import { currentMonitor } from '@tauri-apps/api/window';
const monitor = await currentMonitor();
```


#### [Since](#since-16)

1.0.0

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2454](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2454)

* * *

### [cursorPosition()](#cursorposition)

```

function cursorPosition(): Promise<PhysicalPosition>
```


Get the cursor position relative to the top-left hand corner of the desktop.

Note that the top-left hand corner of the desktop is not necessarily the same as the screen. If the user uses a desktop with multiple monitors, the top-left hand corner of the desktop is the top-left hand corner of the main monitor on Windows and macOS or the top-left of the leftmost monitor on X11.

The coordinates can be negative if the top-left hand corner of the window is outside of the visible screen region.

#### [Returns](#returns-88)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`PhysicalPosition`](about:/reference/javascript/api/namespacedpi/#physicalposition)\>

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2520](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2520)

* * *

### [getAllWindows()](#getallwindows)

```

function getAllWindows(): Promise<Window[]>
```


Gets a list of instances of `Window` for all available windows.

#### [Returns](#returns-89)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`Window`](about:/reference/javascript/api/namespacewindow/#window)\[\]>

#### [Since](#since-17)

1.0.0

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L228](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L228)

* * *

### [getCurrentWindow()](#getcurrentwindow)

```

function getCurrentWindow(): Window
```


Get an instance of `Window` for the current window.

#### [Returns](#returns-90)

[`Window`](about:/reference/javascript/api/namespacewindow/#window)

#### [Since](#since-18)

1.0.0

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L216](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L216)

* * *

### [monitorFromPoint()](#monitorfrompoint)

```

function monitorFromPoint(x, y): Promise<Monitor | null>
```


Returns the monitor that contains the given point. Returns `null` if can’t find any.

#### [Parameters](#parameters-49)


|Parameter|Type  |
|---------|------|
|x        |number|
|y        |number|


#### [Returns](#returns-91)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`Monitor`](about:/reference/javascript/api/namespacewindow/#monitor) | `null`\>

#### [Example](#example-78)

```

import { monitorFromPoint } from '@tauri-apps/api/window';
const monitor = await monitorFromPoint(100.0, 200.0);
```


#### [Since](#since-19)

1.0.0

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2487](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2487)

* * *

### [primaryMonitor()](#primarymonitor)

```

function primaryMonitor(): Promise<Monitor | null>
```


Returns the primary monitor of the system. Returns `null` if it can’t identify any monitor as a primary one.

#### [Returns](#returns-92)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`Monitor`](about:/reference/javascript/api/namespacewindow/#monitor) | `null`\>

#### [Example](#example-79)

```

import { primaryMonitor } from '@tauri-apps/api/window';
const monitor = await primaryMonitor();
```


#### [Since](#since-20)

1.0.0

**Source**: [https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2471](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/window.ts#L2471)

© 2025 Tauri Contributors. CC-BY / MIT