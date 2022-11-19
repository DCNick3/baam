//
// ---------- СПАСИ И СОХРАНИ ----------
//

class CameraList {
  // Store sorted list of all observed cameras

  devices_unknown: string[];
  devices_back: string[];
  devices_front: string[];
  next_id: number;

  constructor() {
    this.devices_unknown = [];
    this.devices_back = []; // list of back cameras
    this.devices_front = []; // list of front cameras

    // [0, this.devices_back.length) - back
    // [this.devices_back.length; this.devices_back.length + this.devices_front.length) - front
    this.next_id = 0;
  }

  get length() {
    return this.devices_back.length + this.devices_front.length + this.devices_unknown.length;
  }

  push_unknown(device_id: string) {
    const unknown_i = this.devices_unknown.indexOf(device_id);
    const back_i = this.devices_back.indexOf(device_id);
    const front_i = this.devices_front.indexOf(device_id);
    if (unknown_i >= 0 || back_i >= 0 || front_i >= 0) return;
    this.devices_unknown.push(device_id);
  }

  push(device_id: string, is_back: boolean) {
    // Check if element are in the correct array
    const unknown_i = this.devices_unknown.indexOf(device_id);
    const back_i = this.devices_back.indexOf(device_id);
    const front_i = this.devices_front.indexOf(device_id);
    if (is_back && back_i >= 0 && front_i < 0 && unknown_i < 0) return;
    if (!is_back && back_i < 0 && front_i >= 0 && unknown_i < 0) return;

    // Deleting element from all lists
    if (unknown_i >= 0) {
      this.devices_unknown.splice(unknown_i, 1); // remove
    }
    if (back_i >= 0) {
      this.devices_back.splice(back_i, 1); // remove
      if (this.next_id > back_i) this.next_id--;
    }
    if (front_i >= 0) {
      this.devices_front.splice(front_i, 1); // remove
      if (this.next_id > front_i) this.next_id--;
    }

    // Insert element
    if (is_back) {
      let i = this.devices_back.length;
      if (this.next_id < i) i = this.next_id;
      this.devices_back.splice(i, 0, device_id); // insert
      this.next_id++;
    } else {
      let i = this.next_id;
      if (i < this.devices_back.length) i = this.length;
      this.devices_front.splice(i, 0, device_id); // insert
      if (i >= this.next_id) this.next_id++;
    }

    this.next_id %= this.length;
  }

  has_unknown() {
    return this.devices_unknown.length > 0;
  }

  get_unknown() {
    return this.devices_unknown[this.devices_unknown.length - 1];
  }

  get_next() {
    let res;
    if (this.next_id < this.devices_back.length) res = this.devices_back[this.next_id];
    else res = this.devices_front[this.next_id - this.devices_back.length];
    this.next_id = (this.next_id + 1) % this.length;
    return res;
  }
}

function convert_error(err: unknown) {
  if (err instanceof Error) return err;
  if (typeof err === 'string') return new Error(err);
  if (err instanceof Object) return new Error(JSON.stringify(err));
  return new Error(String(err));
}

function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

interface InputDeviceInfoExt extends InputDeviceInfo {
  prototype: InputDeviceInfo;
  getCapabilities?(): MediaTrackCapabilities;
}

export enum MediaStreamManagerErrorKind {
  NotSupported = 'NotSupported',
  NoCamera = 'NoCamera',
  AllDevicesFailed = 'AllDevicesFailed',
  PermissionDenied = 'PermissionDenied',
  Other = 'Other'
}

export class MediaStreamManagerError extends Error {
  reason?: Error;
  kind: MediaStreamManagerErrorKind;

  constructor(kind: MediaStreamManagerErrorKind, reason?: Error) {
    super();
    this.kind = kind;
    this.reason = reason;
  }

  toString() {
    return `${this.kind}: ${this.reason?.name}: ${this.reason?.message}`;
  }
}

export class MediaStreamManager {
  #first_open: boolean;
  #device_list: CameraList;
  #stream?: MediaStream;

  constructor() {
    this.#first_open = true;
    this.#device_list = new CameraList();
  }

  get is_supported() {
    // read end of https://developer.mozilla.org/en/docs/Web/API/MediaDevices/getUserMedia
    // see https://github.com/xdumaine/enumerateDevices
    // typescript magic, because it thinks we are stupid to check for existence of getUserMedia & enumerateDevices
    const mediaDevices = <{ [key: string]: unknown } | undefined>(<unknown>navigator.mediaDevices);
    return mediaDevices && mediaDevices.getUserMedia && mediaDevices.enumerateDevices;
  }

  get number_of_devices() {
    return this.#device_list.length;
  }

  async close() {
    if (this.#stream === undefined) return;
    // sleep is very important! Do not remove! See https://gitlab.com/inno_baam/baam/-/issues/94
    // (extract):
    //    На некоторых телефонах страница ScanQRCode зависает вместе со всем браузером.
    //    Проявляется в 100% случаях после нажатия на переключение камеры либо сразу после сканирования кода.
    //    Но только на некоторых телефонах (примерно 2% телефонов).
    //
    //    Причина ошибка находится в реализации самого браузера.
    //    Устранить ее самостоятельно не представляется возможным.
    //    Зависание вызывается командой закрытия камеры.
    await sleep(1000);
    this.#stream.getTracks().forEach((track) => track.stop());
    this.#stream = undefined;
  }

  async getUserMedia(constrains: MediaTrackConstraints, repeat = 10) {
    constrains.width = { ideal: 1920 };
    constrains.height = { ideal: 1080 };

    for (let i = repeat; i >= 0; i--) {
      try {
        return await navigator.mediaDevices.getUserMedia({ audio: false, video: constrains });
      } catch (e) {
        const error = convert_error(e);
        // On some devices android stack returns an error when we try to reopen the device too fast
        if (error.name === 'AbortError' && i > 0) {
          // TODO: find a better way to log?
          console.warn(
            `[Camera] get an AbortError. Wait 1000ms before retry ${i + 1}/${repeat}`,
            error
          );
          await sleep(1000);
          continue;
        }
        throw error;
      }
    }

    throw new Error(`getUserMedia failed after ${repeat} retries`);
  }

  async open_next() {
    await this.close();

    if (!this.is_supported) {
      throw new MediaStreamManagerError(MediaStreamManagerErrorKind.NotSupported);
    }

    if (this.#first_open) {
      // Open any camera
      try {
        this.#stream = await this.getUserMedia({ facingMode: { ideal: 'environment' } }, 0);
        const settings = this.#stream.getTracks()[0].getSettings();

        if (settings.deviceId === undefined) {
          // noinspection ExceptionCaughtLocallyJS
          throw new Error('deviceId is undefined');
        }

        this.#device_list.push(settings.deviceId, settings.facingMode === 'environment');

        return this.#stream;
      } catch (e) {
        if (e instanceof Error) {
          if (e.name === 'NotAllowedError') {
            throw new MediaStreamManagerError(MediaStreamManagerErrorKind.PermissionDenied, e);
          } else if (e.name === 'NotFoundError') {
            throw new MediaStreamManagerError(MediaStreamManagerErrorKind.NoCamera, e);
          } else {
            throw new MediaStreamManagerError(MediaStreamManagerErrorKind.Other, e);
          }
        } else {
          // don't know what to do with unknown error
          throw e;
        }
      }
    }

    // Enumerate all devices
    (await navigator.mediaDevices.enumerateDevices()).forEach((m) => {
      if (m.kind !== 'videoinput') return;
      if (m instanceof InputDeviceInfo) {
        const m_ext = <InputDeviceInfoExt>m;
        if (m_ext.getCapabilities) {
          const caps = m_ext.getCapabilities();
          if (caps.facingMode)
            this.#device_list.push(
              m.deviceId,
              caps.facingMode.some((v) => v === 'environment')
            );
          return;
        }
      }
      this.#device_list.push_unknown(m.deviceId);
    });

    if (this.#first_open) {
      this.#first_open = false;
      return;
    }

    while (this.#device_list.has_unknown()) {
      const device_id = this.#device_list.get_unknown();
      try {
        this.#stream = await this.getUserMedia({
          deviceId: { exact: device_id },
          facingMode: { exact: 'environment' }
        });
        this.#device_list.push(device_id, true);
        return this.#stream;
      } catch (e) {
        const error = convert_error(e);
        if (error.name === 'OverconstrainedError' || error.name === 'ConstraintNotSatisfiedError') {
          this.#device_list.push(device_id, false);
        } else {
          this.#device_list.push(device_id, true);
          console.warn(
            `[Camera] Error while opening environment camera with id=${device_id}. Trying next camera`,
            error
          );
        }
      }
    }

    // Open next camera
    for (let i = 0; i < this.#device_list.length; i++) {
      const device_id = this.#device_list.get_next();
      try {
        this.#stream = await this.getUserMedia({ deviceId: { exact: device_id } });
        const settings = this.#stream.getTracks()[0].getSettings();
        this.#device_list.push(device_id, settings.facingMode === 'environment');
        return this.#stream;
      } catch (e) {
        const error = convert_error(e);

        console.warn(
          `[Camera] Error while opening camera with id=${device_id}. Trying next camera`,
          error
        );
        this.#device_list.push(device_id, false);
      }
    }

    throw new MediaStreamManagerError(MediaStreamManagerErrorKind.AllDevicesFailed, undefined);
  }
}
