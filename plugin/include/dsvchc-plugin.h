#ifndef DSVCHC_PLUGIN_H
#define DSVCHC_PLUGIN_H

#include <stddef.h>
#include <stdint.h>

#define DSVCHC_PLUGIN_ERR_CODE_OK 0
#define DSVCHC_PLUGIN_ERR_CODE_FILE_NOT_FOUND 1
#define DSVCHC_PLUGIN_ERR_CODE_INVALID_CONFIG 2
#define DSVCHC_PLUGIN_ERR_CODE_CHECK_FAIL 3

typedef struct
{
    size_t size;
    const uint8_t *data;
}
dsvchc_plugin_str_t;

#define dsvchc_plugin_const_str(s) \
    { .size = sizeof(s) - 1, .data = (const uint8_t *)(s) }

void *
dsvchc_plugin_ctx_create();

int32_t
dsvchc_plugin_ctx_initialize(void *ctx,
                             const dsvchc_plugin_str_t *conf_path);

void *
dsvchc_plugin_check_create(void *ctx);

int32_t
dsvchc_plugin_check_initialize(void *check,
                               const dsvchc_plugin_str_t *conf_path);

int32_t
dsvchc_plugin_check_perform(void *check);

const dsvchc_plugin_str_t *
dsvchc_plugin_check_get_result_json(void *check);

int32_t
dsvchc_plugin_check_finalize(void *check);

const dsvchc_plugin_str_t *
dsvchc_plugin_check_get_error(void *check);

void
dsvchc_plugin_check_destroy(void *check);

int32_t
dsvchc_plugin_ctx_finalize(void *ctx);

const dsvchc_plugin_str_t *
dsvchc_plugin_ctx_get_error(void *ctx);

void
dsvchc_plugin_ctx_destroy(void *ctx);

#endif //DSVCHC_PLUGIN_H
