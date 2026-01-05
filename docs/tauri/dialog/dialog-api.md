Title: @tauri-apps/plugin-dialog | Tauri

Description: The cross-platform app building toolkit

Skip to content

# @tauri-apps/plugin-dialog

## Interfaces

Section titled “Interfaces”

### ConfirmDialogOptions

Section titled “ConfirmDialogOptions”

#### Properties

Section titled “Properties”

| Property       | Type     | Description                                        | Defined in                                                                                                |
| -------------- | -------- | -------------------------------------------------- | --------------------------------------------------------------------------------------------------------- | ------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| `cancelLabel?` | `string` | The label of the cancel button.                    | **Source**: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L224 |
| `kind?`        | `"info"` | `"warning"`                                        | `"error"`                                                                                                 | The kind of the dialog. Defaults to `info`. | **Source**: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L220 |
| `okLabel?`     | `string` | The label of the confirm button.                   | **Source**: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L222 |
| `title?`       | `string` | The title of the dialog. Defaults to the app name. | **Source**: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L218 |

### DialogFilter

Section titled “DialogFilter”

Extension filters for the file dialog.

#### Since

Section titled “Since”

2.0.0

#### Properties

Section titled “Properties”

| Property     | Type         | Description                                                                          | Defined in                                                                                               |
| ------------ | ------------ | ------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------- |
| `extensions` | `string`\[\] | Extensions to filter, without a `.` prefix. **Example** `extensions: ['svg', 'png']` | **Source**: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L22 |
| `name`       | `string`     | Filter name.                                                                         | **Source**: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L14 |

### MessageDialogOptions

Section titled “MessageDialogOptions”

#### Since

Section titled “Since”

2.0.0

#### Properties

Section titled “Properties”

| Property   | Type                   | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | Defined in                                                                                                |
| ---------- | ---------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- | ------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| `buttons?` | `MessageDialogButtons` | The buttons of the dialog. **Example** `// Use system default buttons texts await message('Hello World!', { buttons: 'Ok' }) await message('Hello World!', { buttons: 'OkCancel' }) // Or with custom button texts await message('Hello World!', { buttons: { ok: 'Yes!' } }) await message('Take on the task?', { buttons: { ok: 'Accept', cancel: 'Cancel' } }) await message('Show the file content?', { buttons: { yes: 'Show content', no: 'Show in folder', cancel: 'Cancel' } })` **Since** 2.4.0 | **Source**: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L190 |
| `kind?`    | `"info"`               | `"warning"`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | `"error"`                                                                                                 | The kind of the dialog. Defaults to `info`. | **Source**: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L161 |
| `okLabel?` | `string`               | The label of the Ok button. **Deprecated** Use `MessageDialogOptions.buttons` instead.                                                                                                                                                                                                                                                                                                                                                                                                                   | **Source**: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L167 |
| `title?`   | `string`               | The title of the dialog. Defaults to the app name.                                                                                                                                                                                                                                                                                                                                                                                                                                                       | **Source**: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L159 |

### OpenDialogOptions

Section titled “OpenDialogOptions”

Options for the open dialog.

#### Since

Section titled “Since”

2.0.0

#### Properties

Section titled “Properties”

| Property                | Type               | Description                                                                                                                                                                                                                                                                                                                                                                                          | Defined in                                                                                               |
| ----------------------- | ------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| `canCreateDirectories?` | `boolean`          | Whether to allow creating directories in the dialog. Enabled by default. **macOS Only**                                                                                                                                                                                                                                                                                                              | **Source**: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L54 |
| `defaultPath?`          | `string`           | Initial directory or file path. If it’s a directory path, the dialog interface will change to that folder. If it’s not an existing directory, the file name will be set to the dialog’s file name input and the dialog will be set to the parent folder. On mobile the file name is always used on the dialog’s file name input. If not provided, Android uses `(invalid).txt` as default file name. | **Source**: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L43 |
| `directory?`            | `boolean`          | Whether the dialog is a directory selection or not.                                                                                                                                                                                                                                                                                                                                                  | **Source**: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L47 |
| `filters?`              | `DialogFilter`\[\] | The filters of the dialog.                                                                                                                                                                                                                                                                                                                                                                           | **Source**: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L34 |
| `multiple?`             | `boolean`          | Whether the dialog allows multiple selection or not.                                                                                                                                                                                                                                                                                                                                                 | **Source**: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L45 |
| `recursive?`            | `boolean`          | If `directory` is true, indicates that it will be read recursively later. Defines whether subdirectories will be allowed on the scope or not.                                                                                                                                                                                                                                                        | **Source**: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L52 |
| `title?`                | `string`           | The title of the dialog window (desktop only).                                                                                                                                                                                                                                                                                                                                                       | **Source**: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L32 |

### SaveDialogOptions

Section titled “SaveDialogOptions”

Options for the save dialog.

#### Since

Section titled “Since”

2.0.0

#### Properties

Section titled “Properties”

| Property                | Type               | Description                                                                                                                                                                                                                                                                                                                                                                                          | Defined in                                                                                               |
| ----------------------- | ------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| `canCreateDirectories?` | `boolean`          | Whether to allow creating directories in the dialog. Enabled by default. **macOS Only**                                                                                                                                                                                                                                                                                                              | **Source**: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L77 |
| `defaultPath?`          | `string`           | Initial directory or file path. If it’s a directory path, the dialog interface will change to that folder. If it’s not an existing directory, the file name will be set to the dialog’s file name input and the dialog will be set to the parent folder. On mobile the file name is always used on the dialog’s file name input. If not provided, Android uses `(invalid).txt` as default file name. | **Source**: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L75 |
| `filters?`              | `DialogFilter`\[\] | The filters of the dialog.                                                                                                                                                                                                                                                                                                                                                                           | **Source**: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L66 |
| `title?`                | `string`           | The title of the dialog window (desktop only).                                                                                                                                                                                                                                                                                                                                                       | **Source**: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L64 |

## Type Aliases

Section titled “Type Aliases”

### MessageDialogButtons

Section titled “MessageDialogButtons”

```
type MessageDialogButtons: MessageDialogDefaultButtons | MessageDialogCustomButtons;
```

The buttons of a message dialog.

#### Since

Section titled “Since”

2.4.0

**Source**: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L150

### MessageDialogButtonsOk

Section titled “MessageDialogButtonsOk”

```
type MessageDialogButtonsOk: object & BanExcept<"ok">;
```

The Ok button of a message dialog.

#### Type declaration

Section titled “Type declaration”

| Name | Type     | Description    | Defined in                                                                                                |
| ---- | -------- | -------------- | --------------------------------------------------------------------------------------------------------- |
| `ok` | `string` | The Ok button. | **Source**: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L132 |

#### Since

Section titled “Since”

2.4.0

**Source**: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L130

### MessageDialogButtonsOkCancel

Section titled “MessageDialogButtonsOkCancel”

```
type MessageDialogButtonsOkCancel: object & BanExcept<"ok" | "cancel">;
```

The Ok and Cancel buttons of a message dialog.

#### Type declaration

Section titled “Type declaration”

| Name     | Type     | Description        | Defined in                                                                                                |
| -------- | -------- | ------------------ | --------------------------------------------------------------------------------------------------------- |
| `cancel` | `string` | The Cancel button. | **Source**: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L122 |
| `ok`     | `string` | The Ok button.     | **Source**: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L120 |

#### Since

Section titled “Since”

2.4.0

**Source**: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L118

### MessageDialogButtonsYesNoCancel

Section titled “MessageDialogButtonsYesNoCancel”

```
type MessageDialogButtonsYesNoCancel: object & BanExcept<"yes" | "no" | "cancel">;
```

The Yes, No and Cancel buttons of a message dialog.

#### Type declaration

Section titled “Type declaration”

| Name     | Type     | Description        | Defined in                                                                                                |
| -------- | -------- | ------------------ | --------------------------------------------------------------------------------------------------------- |
| `cancel` | `string` | The Cancel button. | **Source**: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L110 |
| `no`     | `string` | The No button.     | **Source**: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L108 |
| `yes`    | `string` | The Yes button.    | **Source**: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L106 |

#### Since

Section titled “Since”

2.4.0

**Source**: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L104

### MessageDialogCustomButtons

Section titled “MessageDialogCustomButtons”

```
type MessageDialogCustomButtons: MessageDialogButtonsYesNoCancel | MessageDialogButtonsOkCancel | MessageDialogButtonsOk;
```

Custom buttons for a message dialog.

#### Since

Section titled “Since”

2.4.0

**Source**: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L140

### MessageDialogDefaultButtons

Section titled “MessageDialogDefaultButtons”

```
type MessageDialogDefaultButtons: "Ok" | "OkCancel" | "YesNo" | "YesNoCancel";
```

Default buttons for a message dialog.

#### Since

Section titled “Since”

2.4.0

**Source**: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L85

### MessageDialogResult

Section titled “MessageDialogResult”

```
type MessageDialogResult:  | "Yes"  | "No"  | "Ok"  | "Cancel"  | string & object;
```

The result of a message dialog.

The result is a string if the dialog has custom buttons, otherwise it is one of the default buttons.

#### Since

Section titled “Since”

2.4.0

**Source**: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L337

### OpenDialogReturn<T>

Section titled “OpenDialogReturn<T>”

```
type OpenDialogReturn<T>: T["directory"] extends true ? T["multiple"] extends true ? string[] | null : string | null : T["multiple"] extends true ? string[] | null : string | null;
```

#### Type Parameters

Section titled “Type Parameters”

| Type Parameter                    |
| --------------------------------- |
| `T` _extends_ `OpenDialogOptions` |

**Source**: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L227

## Functions

Section titled “Functions”

### ask()

Section titled “ask()”

```
function ask(message, options?): Promise<boolean>
```

Shows a question dialog with `Yes` and `No` buttons.

#### Parameters

Section titled “Parameters”

| Parameter  | Type     | Description            |
| ---------- | -------- | ---------------------- | ------------------------------------------------------------------ |
| `message`  | `string` | The message to show.   |
| `options`? | `string` | `ConfirmDialogOptions` | The dialog’s options. If a string, it represents the dialog title. |

#### Returns

Section titled “Returns”

`Promise`<`boolean`\>

A promise resolving to a boolean indicating whether `Yes` was clicked or not.

#### Example

Section titled “Example”

```
import { ask } from '@tauri-apps/plugin-dialog';const yes = await ask('Are you sure?', 'Tauri');const yes2 = await ask('This action cannot be reverted. Are you sure?', { title: 'Tauri', kind: 'warning' });
```

#### Since

Section titled “Since”

2.0.0

**Source**: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L387

### confirm()

Section titled “confirm()”

```
function confirm(message, options?): Promise<boolean>
```

Shows a question dialog with `Ok` and `Cancel` buttons.

#### Parameters

Section titled “Parameters”

| Parameter  | Type     | Description            |
| ---------- | -------- | ---------------------- | ------------------------------------------------------------------ |
| `message`  | `string` | The message to show.   |
| `options`? | `string` | `ConfirmDialogOptions` | The dialog’s options. If a string, it represents the dialog title. |

#### Returns

Section titled “Returns”

`Promise`<`boolean`\>

A promise resolving to a boolean indicating whether `Ok` was clicked or not.

#### Example

Section titled “Example”

```
import { confirm } from '@tauri-apps/plugin-dialog';const confirmed = await confirm('Are you sure?', 'Tauri');const confirmed2 = await confirm('This action cannot be reverted. Are you sure?', { title: 'Tauri', kind: 'warning' });
```

#### Since

Section titled “Since”

2.0.0

**Source**: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L417

### message()

Section titled “message()”

```
function message(message, options?): Promise<MessageDialogResult>
```

Shows a message dialog with an `Ok` button.

#### Parameters

Section titled “Parameters”

| Parameter  | Type     | Description            |
| ---------- | -------- | ---------------------- | ------------------------------------------------------------------ |
| `message`  | `string` | The message to show.   |
| `options`? | `string` | `MessageDialogOptions` | The dialog’s options. If a string, it represents the dialog title. |

#### Returns

Section titled “Returns”

`Promise`<`MessageDialogResult`\>

A promise indicating the success or failure of the operation.

#### Example

Section titled “Example”

```
import { message } from '@tauri-apps/plugin-dialog';await message('Tauri is awesome', 'Tauri');await message('File not found', { title: 'Tauri', kind: 'error' });
```

#### Since

Section titled “Since”

2.0.0

**Source**: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L356

### open()

Section titled “open()”

```
function open<T>(options): Promise<OpenDialogReturn<T>>
```

Open a file/directory selection dialog.

The selected paths are added to the filesystem and asset protocol scopes. When security is more important than the easy of use of this API, prefer writing a dedicated command instead.

Note that the scope change is not persisted, so the values are cleared when the application is restarted. You can save it to the filesystem using tauri-plugin-persisted-scope.

#### Type Parameters

Section titled “Type Parameters”

| Type Parameter                    |
| --------------------------------- |
| `T` _extends_ `OpenDialogOptions` |

#### Parameters

Section titled “Parameters”

| Parameter | Type |
| --------- | ---- |
| `options` | `T`  |

#### Returns

Section titled “Returns”

`Promise`<`OpenDialogReturn`<`T`\>>

A promise resolving to the selected path(s)

#### Examples

Section titled “Examples”

```
import { open } from '@tauri-apps/plugin-dialog';// Open a selection dialog for image filesconst selected = await open({  multiple: true,  filters: [{    name: 'Image',    extensions: ['png', 'jpeg']  }]});if (Array.isArray(selected)) {  // user selected multiple files} else if (selected === null) {  // user cancelled the selection} else {  // user selected a single file}
```

```
import { open } from '@tauri-apps/plugin-dialog';import { appDir } from '@tauri-apps/api/path';// Open a selection dialog for directoriesconst selected = await open({  directory: true,  multiple: true,  defaultPath: await appDir(),});if (Array.isArray(selected)) {  // user selected multiple directories} else if (selected === null) {  // user cancelled the selection} else {  // user selected a single directory}
```

#### Since

Section titled “Since”

2.0.0

**Source**: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L287

### save()

Section titled “save()”

```
function save(options): Promise<string | null>
```

Open a file/directory save dialog.

The selected path is added to the filesystem and asset protocol scopes. When security is more important than the easy of use of this API, prefer writing a dedicated command instead.

Note that the scope change is not persisted, so the values are cleared when the application is restarted. You can save it to the filesystem using tauri-plugin-persisted-scope.

#### Parameters

Section titled “Parameters”

| Parameter | Type                |
| --------- | ------------------- |
| `options` | `SaveDialogOptions` |

#### Returns

Section titled “Returns”

`Promise`<`string` | `null`\>

A promise resolving to the selected path.

#### Example

Section titled “Example”

```
import { save } from '@tauri-apps/plugin-dialog';const filePath = await save({  filters: [{    name: 'Image',    extensions: ['png', 'jpeg']  }]});
```

#### Since

Section titled “Since”

2.0.0

**Source**: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/dialog/guest-js/index.ts#L321

Support on Open Collective Sponsor on GitHub

© 2025 Tauri Contributors. CC-BY / MIT
