#ifndef COMMUNICATOR_H
#define COMMUNICATOR_H

#include <sys/types.h>
#include <stddef.h>
#include <stdint.h>

/* Forward declarations */
typedef struct communicator_t communicator_t;

typedef struct communicator_t
{
    /* db_addoncom_sender_t interface */
    int (*open)(communicator_t* self, const char* path);
    int (*close)(communicator_t* self);
    ssize_t (*send)(communicator_t* self, const uint8_t* buf, size_t len);
    /* Extra receive function */
    ssize_t (*recv)(communicator_t* self, uint8_t* buf, size_t len);
} communicator_t;

#endif /* COMMUNICATOR_H */
