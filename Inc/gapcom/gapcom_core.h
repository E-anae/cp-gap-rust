#ifndef GAPCOM_CORE_H
#define GAPCOM_CORE_H

#include "TinyFrame.h"
#include "gapcom/gapcom.h"

#include <stdint.h>

//----------------------------- FRAME FORMAT ---------------------------------
// The TinyFrame header and footer encapsulate a Protocol Buffer payload.
//
// ,-----+-----+-----+------+------------+- - - - - -+-------------,
// | SOF | ID  | LEN | TYPE | HEAD_CKSUM | PROTOBUF  | DATA_CKSUM  |
// | 0-1 | 1-4 | 1-4 | 1-4  | 0-4        |    ...    | 0-4         |
// '-----+-----+-----+------+------------+- - - - - -+-------------'
//
#define GAPCOM_TF_CHECKSUM_SIZE_BYTES      \
    (                                           \
        (TF_CKSUM_TYPE == TF_CKSUM_CRC8)  ? 1 : \
        (TF_CKSUM_TYPE == TF_CKSUM_CRC16) ? 2 : \
        (TF_CKSUM_TYPE == TF_CKSUM_CRC32) ? 4 : \
                                            0)
#define GAPCOM_TF_HEADER_SIZE_BYTES \
    (                                    \
        TF_USE_SOF_BYTE +                \
        TF_ID_BYTES +                    \
        TF_LEN_BYTES +                   \
        TF_TYPE_BYTES +                  \
        GAPCOM_TF_CHECKSUM_SIZE_BYTES)
#define GAPCOM_PROTO_PAYLOAD_MAXSIZE_BYTES (4192)
#define GAPCOM_TF_FOOTER_SIZE_BYTES        GAPCOM_TF_CHECKSUM_SIZE_BYTES

/* Send buffer only needs to hold the Protocol Buffer data. The TinyFrame
 * header/footer are handled by the TinyFrame library itself. TinyFrame calls
 * the communication backend's send function.
 */
#define GAPCOM_SENDBUF_SIZE GAPCOM_PROTO_PAYLOAD_MAXSIZE_BYTES
/* Receive buffer needs to hold both the Protocol Buffer data and the TinyFrame
 * header/footer. This is because we are the ones calling the communication
 * backend's receive function, which will give us the full frame.
 */
#define GAPCOM_RECVBUF_SIZE           \
    (                                      \
        GAPCOM_TF_HEADER_SIZE_BYTES + \
        GAPCOM_TF_FOOTER_SIZE_BYTES + \
        GAPCOM_PROTO_PAYLOAD_MAXSIZE_BYTES)

#define GAPCOM_TF_NO_TIMEOUT (0)

typedef struct gapcom_handle_t
{
    TinyFrame tf_handle;
    TF_Msg tf_msg;
    gapcom_sender_t* sender_impl;
    gapcom_callback_t user_callback[GAPCOM_MSG_COUNT];
    uint8_t sendbuf[GAPCOM_SENDBUF_SIZE];
} gapcom_handle_t;

/**
 * container_of - cast a member of a structure out to the containing structure
 * @ptr:	the pointer to the member.
 * @type:	the type of the container struct this is embedded in.
 * @member:	the name of the member within the struct.
 *
 * WARNING: any const qualifier of @ptr is lost.
 */
#define container_of(ptr, type, member) ({            \
    const typeof(((type*)0)->member)* __mptr = (ptr); \
    (type*)((char*)__mptr - offsetof(type, member));  \
})

/**
 * @brief Tell TinyFrame how to actually send a frame.
 *
 * @param tf_handle Handle to TinyFrame session
 * @param buf Frame buffer
 * @param len Length of frame buffer
 */
void TF_WriteImpl(TinyFrame* tf_handle, const uint8_t* buf, uint32_t len);

TF_Result gapcom_tf_id_listener(TinyFrame* tf_handle, TF_Msg* tf_msg);

/* CAUTION : The stream.bytes_written cast won't work with payloads larger than
 * INTMAX but we're designing this library for talking with a limited MCU,
 * using small messages. We assume such large messages will never be sent.
 */
#define gapcom_encapsulate_protobuf(handle, proto_fields, proto_obj)             \
    ({                                                                               \
        gapcom_handle_t* _handle = (handle);                                    \
        pb_ostream_t stream;                                                         \
        bool status;                                                                 \
        int encoded_bytes;                                                           \
                                                                                     \
        stream = pb_ostream_from_buffer(_handle->sendbuf, sizeof(_handle->sendbuf)); \
        status = pb_encode(&stream, (proto_fields), &(proto_obj));                   \
        if (status)                                                                  \
            encoded_bytes = (int)(stream.bytes_written);                             \
        else                                                                         \
            encoded_bytes = -1;                                                      \
        encoded_bytes;                                                               \
    })

int gapcom_encapsulate_tf(
    gapcom_handle_t* handle,
    gapcom_msg_t msg_type,
    int msg_len);

#endif /* GAPCOM_CORE_H */
