const ERRORS = {
  UnknownError: 0,
  ShowSaveFilePickerAbort: 1,
  ShowSaveFilePickerSecurity: 2,
  ShowSaveFilePickerType: 3,
  CreateWritableNotAllowed: 4,
  CreateWritableNotFound: 5,
  CreateWritableNoModificationAllowed: 6,
  CreateWritableAbort: 7,
  WriteNotAllowed: 8,
  WriteQuotaExceeded: 9,
  WriteType: 10,
  CloseType: 11,
};
const SHOW_SAVE_FILE_PICKER_ERRORS = {
  AbortError: ERRORS.ShowSaveFilePickerAbort,
  SecurityErrror: ERRORS.ShowSaveFilePickerSecurity,
  TypeError: ERRORS.ShowSaveFilePickerType,
};
const CREATE_WRITABLE_ERRORS = {
  NotAllowed: ERRORS.CreateWritableNotAllowed,
  NotFound: ERRORS.CreateWritableNotFound,
  NoModificationAllowed: ERRORS.CreateWritableNoModificationAllowed,
  Abort: ERRORS.CreateWritableAbort,
};
const WRITE_ERRORS = {
  NotAllowed: ERRORS.WriteNotAllowed,
  QuotaExceeded: ERRORS.QuotaExceeded,
  TypeError: ERRORS.WriteType,
};
const CLOSE_ERRORS = {
  TypeError: ERRORS.CloseType,
};

function matchErr(errors, err) {
  let err = errors[err.name];
  if (Number.isInteger(err)) {
    return err;
  }
  return ERRORS.UnknownError;
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

async function save_to_file(handle, file_data) {
  if (handle === null) {
    handle = showSaveFilePicker();
  }

  let stream = createWritable(handle);

  await write(stream);
  await close(stream);

  return handle;
}
