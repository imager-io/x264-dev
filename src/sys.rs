#![allow(non_upper_case_globals)]
use std::ffi::{CString, c_void};
use std::os::raw::{c_char, c_int};
use libc::{size_t, c_float};

// use crate::raw::{
//     x264_t,
//     x264_nal_t,
//     x264_zone_t,
//     x264_param_t,
//     x264_level_t,
//     x264_hrd_t,
//     x264_sei_payload_t,
//     x264_sei_t,
//     x264_image_t,
//     x264_image_properties_t,
//     x264_picture_t,
//     x264_param_t__bindgen_ty_1,
//     x264_param_t__bindgen_ty_2,
//     x264_param_t__bindgen_ty_3,
//     x264_param_t__bindgen_ty_4,

//     x264_direct_pred_names,
//     x264_motion_est_names,
//     x264_b_pyramid_names,
//     x264_overscan_names,
//     x264_vidformat_names,
//     x264_fullrange_names,
//     x264_colorprim_names,
//     x264_transfer_names,
//     x264_colmatrix_names,
//     x264_nal_hrd_names,
//     x264_avcintra_flavor_names,
//     x264_levels,
//     x264_preset_names,
//     x264_tune_names,
//     x264_profile_names,
//     x264_chroma_format,
// };


///////////////////////////////////////////////////////////////////////////////
// X264 STRUCTS
///////////////////////////////////////////////////////////////////////////////

/// opaque handler for encoder
pub type X264T = crate::raw::x264_t;

/// The data within the payload is already NAL-encapsulated; the ref_idc and type
/// are merely in the struct for easy access by the calling application.
/// 
/// All data returned in an x264_nal_t, including the data in p_payload, is no longer
/// valid after the next call to x264_encoder_encode.  Thus it must be used or copied
/// before calling x264_encoder_encode or x264_encoder_headers again.
pub type X264NalT = crate::raw::x264_nal_t;

/// Zones: override ratecontrol or other options for specific sections of the video.
/// 
/// See x264_encoder_reconfig() for which options can be changed.
/// If zones overlap, whichever comes later in the list takes precedence.
pub type X264ZoneT = crate::raw::x264_zone_t;

pub type X264ParamT = crate::raw::x264_param_t;

pub type X264LevelT = crate::raw::x264_level_t;

pub type X264HrdT = crate::raw::x264_hrd_t;

/// Arbitrary user SEI:
/// 
/// Payload size is in bytes and the payload pointer must be valid.
/// 
/// Payload types and syntax can be found in Annex D of the H.264 Specification.
/// SEI payload alignment bits as described in Annex D must be included at the
/// end of the payload if needed.
/// The payload should not be NAL-encapsulated.
/// Payloads are written first in order of input, apart from in the case when HRD
/// is enabled where payloads are written after the Buffering Period SEI.
pub type X264SeiPayloadT = crate::raw::x264_sei_payload_t;

pub type X264SeiT = crate::raw::x264_sei_t;

pub type X264ImageT = crate::raw::x264_image_t;

pub type X264ImagePropertiesT = crate::raw::x264_image_properties_t;

pub type X264PictureT = crate::raw::x264_picture_t;


///////////////////////////////////////////////////////////////////////////////
// X264 TYPE-DEFS
///////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////
// X264 CONSTANTS
///////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////
// X264 FUNCTIONS
///////////////////////////////////////////////////////////////////////////////

pub unsafe fn x264_nal_encode(
    h: *mut X264T,
    dst: *mut u8,
    nal: *mut X264NalT,
) {
    crate::raw::x264_nal_encode(
        h,
        dst,
        nal,
    )
}


/// set one parameter by name.
/// 
/// returns 0 on success, or returns one of the following errors.
/// note: BAD_VALUE occurs only if it can't even parse the value,
/// numerical range is not checked until x264_encoder_open() or
/// x264_encoder_reconfig().
/// value=NULL means "true" for boolean options, but is a BAD_VALUE for non-booleans.
pub unsafe fn x264_param_parse(
    arg1: *mut X264ParamT,
    name: *const ::std::os::raw::c_char,
    value: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    crate::raw::x264_param_parse(
        arg1,
        name,
        value,
    )
}

/// Multiple tunings can be used if separated by a delimiter in ",./-+",
/// however multiple psy tunings cannot be used.
/// 
/// film, animation, grain, stillimage, psnr, and ssim are psy tunings.
///
/// returns 0 on success, negative on failure (e.g. invalid preset/tune name).
pub unsafe fn x264_param_default_preset(
    arg1: *mut X264ParamT,
    preset: *const ::std::os::raw::c_char,
    tune: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    crate::raw::x264_param_default_preset(
        arg1,
        preset,
        tune,
    )
}

/// x264_param_apply_fastfirstpass:
/// If first-pass mode is set (rc.b_stat_read == 0, rc.b_stat_write == 1),
/// modify the encoder settings to disable options generally not useful on
/// the first pass.
pub unsafe fn x264_param_apply_fastfirstpass(arg1: *mut X264ParamT) {
    crate::raw::x264_param_apply_fastfirstpass(arg1)
}


/// Applies the restrictions of the given profile.
/// 
/// Currently available profiles are, from most to least restrictive:
pub unsafe fn x264_param_apply_profile(
    arg1: *mut X264ParamT,
    profile: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    crate::raw::x264_param_apply_profile(
        arg1,
        profile,
    )
}

/// initialize an x264_picture_t. 
/// 
/// Needs to be done if the calling application
/// allocates its own x264_picture_t as opposed to using x264_picture_alloc.
pub unsafe fn x264_picture_init(pic: *mut X264PictureT) {
    crate::raw::x264_picture_init(pic)
}

/// alloc data for a picture.
/// 
/// You must call x264_picture_clean on it.
/// returns 0 on success, or -1 on malloc failure or invalid colorspace.
pub unsafe fn x264_picture_alloc(
    pic: *mut X264PictureT,
    i_csp: ::std::os::raw::c_int,
    i_width: ::std::os::raw::c_int,
    i_height: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    crate::raw::x264_picture_alloc(
        pic,
        i_csp,
        i_width,
        i_height,
    )
}

/// free associated resource for a x264_picture_t allocated with
/// x264_picture_alloc ONLY
pub unsafe fn x264_picture_clean(pic: *mut X264PictureT) {
    crate::raw::x264_picture_clean(pic)
}
/// various parameters from x264_param_t are copied.
/// 
/// this takes effect immediately, on whichever frame is encoded next;
/// due to delay, this may not be the next frame passed to encoder_encode.
/// if the change should apply to some particular frame, use x264_picture_t->param instead.
/// returns 0 on success, negative on parameter validation error.
/// not all parameters can be changed; see the actual function for a detailed breakdown.
///
/// since not all parameters can be changed, moving from preset to preset may not always
/// fully copy all relevant parameters, but should still work usably in practice. however,
/// more so than for other presets, many of the speed shortcuts used in ultrafast cannot be
/// switched out of; using reconfig to switch between ultrafast and other presets is not
/// recommended without a more fine-grained breakdown of parameters to take this into account.
pub unsafe fn x264_encoder_reconfig(
    arg1: *mut X264T,
    arg2: *mut X264ParamT,
) -> ::std::os::raw::c_int {
    crate::raw::x264_encoder_reconfig(
        arg1,
        arg2,
    )
}


/// copies the current internal set of parameters to the pointer provided
/// by the caller.
/// 
/// useful when the calling application needs to know
/// how x264_encoder_open has changed the parameters, or the current state
/// of the encoder after multiple x264_encoder_reconfig calls.
/// note that the data accessible through pointers in the returned param struct
/// (e.g. filenames) should not be modified by the calling application.
pub unsafe fn x264_encoder_parameters(
    arg1: *mut X264T,
    arg2: *mut X264ParamT
) {
    crate::raw::x264_encoder_parameters(
        arg1,
        arg2,
    )
}

/// return the SPS and PPS that will be used for the whole stream.
/// 
/// *pi_nal is the number of NAL units outputted in pp_nal.
/// returns the number of bytes in the returned NALs.
/// returns negative on error.
/// the payloads of all output NALs are guaranteed to be sequential in memory.
pub unsafe fn x264_encoder_headers(
    arg1: *mut X264T,
    pp_nal: *mut *mut X264NalT,
    pi_nal: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    crate::raw::x264_encoder_headers(
        arg1,
        pp_nal,
        pi_nal,
    )
}

/// encode one picture.
/// 
/// *pi_nal is the number of NAL units outputted in pp_nal.
/// returns the number of bytes in the returned NALs.
/// returns negative on error and zero if no NAL units returned.
/// the payloads of all output NALs are guaranteed to be sequential in memory.
pub unsafe fn x264_encoder_encode(
    arg1: *mut X264T,
    pp_nal: *mut *mut X264NalT,
    pi_nal: *mut ::std::os::raw::c_int,
    pic_in: *mut X264PictureT,
    pic_out: *mut X264PictureT,
) -> ::std::os::raw::c_int {
    crate::raw::x264_encoder_encode(
        arg1,
        pp_nal,
        pi_nal,
        pic_in,
        pic_out,
    )
}

/// close an encoder handler
pub unsafe fn x264_encoder_close(arg1: *mut X264T) {
    crate::raw::x264_encoder_close(arg1)
}

/// return the number of currently delayed (buffered) frames
/// this should be used at the end of the stream, to know when you have all the encoded frames.
pub unsafe fn x264_encoder_delayed_frames(arg1: *mut X264T) -> ::std::os::raw::c_int {
    crate::raw::x264_encoder_delayed_frames(arg1)
}

/// return the maximum number of delayed (buffered) frames that can occur with the current
/// parameters.
pub unsafe fn x264_encoder_maximum_delayed_frames(arg1: *mut X264T) -> ::std::os::raw::c_int {
    crate::raw::x264_encoder_maximum_delayed_frames(arg1)
}

/// If an intra refresh is not in progress, begin one with the next P-frame.
/// 
/// If an intra refresh is in progress, begin one as soon as the current one finishes.
/// Requires that b_intra_refresh be set.
///
/// Useful for interactive streaming where the client can tell the server that packet loss has
/// occurred.  In this case, keyint can be set to an extremely high value so that intra refreshes
/// only occur when calling x264_encoder_intra_refresh.
///
/// In multi-pass encoding, if x264_encoder_intra_refresh is called differently in each pass,
/// behavior is undefined.
///
/// Should not be called during an x264_encoder_encode.
pub unsafe fn x264_encoder_intra_refresh(arg1: *mut X264T) {
    crate::raw::x264_encoder_intra_refresh(arg1)
}

/// An interactive error resilience tool, designed for use in a low-latency one-encoder-few-clients
/// system.
/// 
/// When the client has packet loss or otherwise incorrectly decodes a frame, the encoder
/// can be told with this command to "forget" the frame and all frames that depend on it, referencing
/// only frames that occurred before the loss.  This will force a keyframe if no frames are left to
/// reference after the aforementioned "forgetting".
///
/// It is strongly recommended to use a large i_dpb_size in this case, which allows the encoder to
/// keep around extra, older frames to fall back on in case more recent frames are all invalidated.
/// Unlike increasing i_frame_reference, this does not increase the number of frames used for motion
/// estimation and thus has no speed impact.  It is also recommended to set a very large keyframe
/// interval, so that keyframes are not used except as necessary for error recovery.
///
/// x264_encoder_invalidate_reference is not currently compatible with the use of B-frames or intra
/// refresh.
///
/// In multi-pass encoding, if x264_encoder_invalidate_reference is called differently in each pass,
/// behavior is undefined.
///
/// Should not be called during an x264_encoder_encode, but multiple calls can be made simultaneously.
///
/// Returns 0 on success, negative on failure. */
pub unsafe fn x264_encoder_invalidate_reference(
    arg1: *mut X264T,
    pts: i64,
) -> ::std::os::raw::c_int {
    crate::raw::x264_encoder_invalidate_reference(
        arg1,
        pts,
    )
}

/// create a new encoder handler, all parameters from x264_param_t are copied
pub unsafe fn x264_encoder_open(arg1: *mut X264ParamT) -> *mut X264T {
    crate::raw::x264_encoder_open_157(arg1)
}



