use std::sync::atomic::{AtomicU32, Ordering};

macro_rules! stub {
    ($name:ident) => {
        #[no_mangle]
        pub extern "system" fn $name() -> u32 {
            STATIC_COUNTER.fetch_add(1, Ordering::Relaxed);
            0
        }
    };
}

static STATIC_COUNTER: AtomicU32 = AtomicU32::new(0);

stub!(CloseDriver);
stub!(DefDriverProc);
stub!(DriverCallback);
stub!(DrvGetModuleHandle);
stub!(GetDriverModuleHandle);
stub!(OpenDriver);
stub!(PlaySound);
stub!(PlaySoundA);
stub!(PlaySoundW);
stub!(SendDriverMessage);
stub!(WOWAppExit);
stub!(auxGetDevCapsA);
stub!(auxGetDevCapsW);
stub!(auxGetNumDevs);
stub!(auxGetVolume);
stub!(auxOutMessage);
stub!(auxSetVolume);
stub!(joyConfigChanged);
stub!(joyGetDevCapsA);
stub!(joyGetDevCapsW);
stub!(joyGetNumDevs);
stub!(joyGetPos);
stub!(joyGetPosEx);
stub!(joyGetThreshold);
stub!(joyReleaseCapture);
stub!(joySetCapture);
stub!(joySetThreshold);
stub!(mciExecute);
stub!(mciFreeCommandResource);
stub!(mciGetCreatorTask);
stub!(mciGetDeviceIDA);
stub!(mciGetDeviceIDW);
stub!(mciGetDeviceIDFromElementIDA);
stub!(mciGetDeviceIDFromElementIDW);
stub!(mciGetDriverData);
stub!(mciGetErrorStringA);
stub!(mciGetErrorStringW);
stub!(mciGetYieldProc);
stub!(mciLoadCommandResource);
stub!(mciSendCommandA);
stub!(mciSendCommandW);
stub!(mciSendStringA);
stub!(mciSendStringW);
stub!(mciSetDriverData);
stub!(mciSetYieldProc);
stub!(mid32Message);
stub!(midiConnect);
stub!(midiDisconnect);
stub!(midiInAddBuffer);
stub!(midiInClose);
stub!(midiInGetDevCapsA);
stub!(midiInGetDevCapsW);
stub!(midiInGetErrorTextA);
stub!(midiInGetErrorTextW);
stub!(midiInGetID);
stub!(midiInGetNumDevs);
stub!(midiInMessage);
stub!(midiInOpen);
stub!(midiInPrepareHeader);
stub!(midiInReset);
stub!(midiInStart);
stub!(midiInStop);
stub!(midiInUnprepareHeader);
stub!(midiOutCacheDrumPatches);
stub!(midiOutCachePatches);
stub!(midiOutClose);
stub!(midiOutGetDevCapsA);
stub!(midiOutGetDevCapsW);
stub!(midiOutGetErrorTextA);
stub!(midiOutGetErrorTextW);
stub!(midiOutGetID);
stub!(midiOutGetNumDevs);
stub!(midiOutGetVolume);
stub!(midiOutLongMsg);
stub!(midiOutMessage);
stub!(midiOutOpen);
stub!(midiOutPrepareHeader);
stub!(midiOutReset);
stub!(midiOutSetVolume);
stub!(midiOutShortMsg);
stub!(midiOutUnprepareHeader);
stub!(midiStreamClose);
stub!(midiStreamOpen);
stub!(midiStreamOut);
stub!(midiStreamPause);
stub!(midiStreamPosition);
stub!(midiStreamProperty);
stub!(midiStreamRestart);
stub!(midiStreamStop);
stub!(mixerClose);
stub!(mixerGetControlDetailsA);
stub!(mixerGetControlDetailsW);
stub!(mixerGetDevCapsA);
stub!(mixerGetDevCapsW);
stub!(mixerGetID);
stub!(mixerGetLineControlsA);
stub!(mixerGetLineControlsW);
stub!(mixerGetLineInfoA);
stub!(mixerGetLineInfoW);
stub!(mixerGetNumDevs);
stub!(mixerMessage);
stub!(mixerOpen);
stub!(mixerSetControlDetails);
stub!(mmDrvInstall);
stub!(mmGetCurrentTask);
stub!(mmTaskBlock);
stub!(mmTaskCreate);
stub!(mmTaskSignal);
stub!(mmTaskYield);
stub!(mmioAdvance);
stub!(mmioAscend);
stub!(mmioClose);
stub!(mmioCreateChunk);
stub!(mmioDescend);
stub!(mmioFlush);
stub!(mmioGetInfo);
stub!(mmioInstallIOProcA);
stub!(mmioInstallIOProcW);
stub!(mmioOpenA);
stub!(mmioOpenW);
stub!(mmioRead);
stub!(mmioRenameA);
stub!(mmioRenameW);
stub!(mmioSeek);
stub!(mmioSendMessage);
stub!(mmioSetBuffer);
stub!(mmioSetInfo);
stub!(mmioStringToFOURCCA);
stub!(mmioStringToFOURCCW);
stub!(mmioWrite);
stub!(sndOpenSound);
stub!(sndPlaySoundA);
stub!(sndPlaySoundW);

// only the 3 functions we care about:
#[no_mangle]
pub extern "system" fn timeGetTime() -> u32 { 0 }

#[no_mangle]
pub extern "system" fn timeBeginPeriod(_p: u32) -> u32 { 0 }

#[no_mangle]
pub extern "system" fn timeEndPeriod(_p: u32) -> u32 { 0 }

