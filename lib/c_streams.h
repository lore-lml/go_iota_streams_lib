#ifndef RUSTC_C_STREAMS_H
#define RUSTC_C_STREAMS_H

#include <stdint.h>

void hello_from_rust(const char *str);

typedef struct ChannelWriter channel_writer_t;

typedef struct ChannelInfo{
    char *channel_id;
    char *announce_id;
} channel_info_t;

typedef struct KeyNonce{
    uint8_t key[32];
    uint8_t nonce[24];
} key_nonce_t;

typedef struct RawPacket raw_packet_t;

extern channel_writer_t *new_channel_writer();
extern channel_info_t const *open_channel_writer(channel_writer_t *);
extern char const *send_raw_data(channel_writer_t *, raw_packet_t const*, key_nonce_t const*);
extern raw_packet_t const *new_raw_packet(uint8_t *pub, uint64_t p_len, uint8_t *mask, uint64_t m_len);
extern void drop_channel_writer(channel_writer_t const *);
extern void drop_channel_info(channel_info_t const *);
extern void drop_str(char const *);
extern char const *hash_string(char const*);
extern key_nonce_t const *create_encryption_key_nonce(char const* key, char const* nonce);
extern void drop_key_nonce(key_nonce_t const *);
extern void drop_raw_packet(raw_packet_t const *);


#endif //RUSTC_C_STREAMS_H
