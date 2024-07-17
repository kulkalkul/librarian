const ERRORS = {
  Unknown: 0,
  ShowSaveUnknown: 1,
  ShowSaveFilePickerAbort: 2,
  ShowSaveFilePickerSecurity: 3,
  ShowSaveFilePickerType: 4,
  CreateWritableUnknown: 5,
  CreateWritableNotAllowed: 6,
  CreateWritableNotFound: 7,
  CreateWritableNoModificationAllowed: 8,
  CreateWritableAbort: 9,
  WriteUnknown: 10,
  WriteNotAllowed: 11,
  WriteQuotaExceeded: 12,
  WriteType: 13,
  CloseUnknown: 14,
  CloseType: 15,
};
const SHOW_SAVE_FILE_PICKER_ERRORS = {
  Unknown: ERRORS.ShowSaveUnknown,
  AbortError: ERRORS.ShowSaveFilePickerAbort,
  SecurityError: ERRORS.ShowSaveFilePickerSecurity,
  TypeError: ERRORS.ShowSaveFilePickerType,
};
const CREATE_WRITABLE_ERRORS = {
  Unknown: ERRORS.CreateWritableUnknown,
  NotAllowedError: ERRORS.CreateWritableNotAllowed,
  NotFoundError: ERRORS.CreateWritableNotFound,
  NoModificationAllowedError: ERRORS.CreateWritableNoModificationAllowed,
  AbortError: ERRORS.CreateWritableAbort,
};
const WRITE_ERRORS = {
  Unknown: ERRORS.WriteUnknown,
  NotAllowedError: ERRORS.WriteNotAllowed,
  QuotaExceededError: ERRORS.QuotaExceeded,
  TypeError: ERRORS.WriteType,
};
const CLOSE_ERRORS = {
  Unknown: ERRORS.CloseUnknown,
  TypeError: ERRORS.CloseType,
};

function matchErr(errors, err) {
  let name = errors[err.name];
  if (Number.isInteger(name)) {
    return name;
  }
  return errors.Unknown;
}

async function showSaveFilePicker() {
  try {
    return await window.showSaveFilePicker();
  } catch (err) {
    throw matchErr(SHOW_SAVE_FILE_PICKER_ERRORS, err);
  }
}

async function createWritable(handle) {
  try {
    return await handle.createWritable();
  } catch (err) {
    throw matchErr(CREATE_WRITABLE_ERRORS, err);
  }
}

async function write(stream, file_data) {
  try {
    await stream.write(file_data);
  } catch (err) {
    throw matchErr(WRITE_ERRORS, err);
  }
}

async function close(stream) {
  try {
    await stream.close();
  } catch (err) {
    throw matchErr(CLOSE_ERRORS, err);
  }
}

export async function save_to_file(handle, file_data) {
  if (!handle) {
    handle = await showSaveFilePicker();
  }

  let stream = await createWritable(handle);

  await write(stream, file_data);
  await close(stream);

  return handle;
}
