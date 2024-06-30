#ifndef DSVCHC_PLUGIN_H
#define DSVCHC_PLUGIN_H

#include <stddef.h>
#include <stdint.h>

#define DSVCHC_PLUGIN_ERR_CODE_OK 0
#define DSVCHC_PLUGIN_ERR_CODE_FILE_NOT_FOUND 1
#define DSVCHC_PLUGIN_ERR_CODE_INVALID_CONFIG 2
#define DSVCHC_PLUGIN_ERR_CODE_CHECK_FAIL 3

#ifdef __cplusplus
extern "C"
{
#endif

typedef struct
{
    size_t size;
    const uint8_t *data;
}
dsvchc_plugin_str_t;

#ifdef __cplusplus
}
#endif

#define dsvchc_plugin_const_str(s) \
    { .size = sizeof(s) - 1, .data = (const uint8_t *)(s) }

#ifdef __cplusplus
extern "C"
#endif
void *
dsvchc_plugin_ctx_create();

#ifdef __cplusplus
extern "C"
#endif
int32_t
dsvchc_plugin_ctx_initialize(void *ctx,
                             const dsvchc_plugin_str_t *conf_path);

#ifdef __cplusplus
extern "C"
#endif
void *
dsvchc_plugin_check_create(void *ctx);

#ifdef __cplusplus
extern "C"
#endif
int32_t
dsvchc_plugin_check_initialize(void *ctx, void *check,
                               const dsvchc_plugin_str_t *conf_path);

#ifdef __cplusplus
extern "C"
#endif
int32_t
dsvchc_plugin_check_perform(void *ctx, void *check);

#ifdef __cplusplus
extern "C"
#endif
const dsvchc_plugin_str_t *
dsvchc_plugin_check_get_result_json(void *ctx, void *check);

#ifdef __cplusplus
extern "C"
#endif
int32_t
dsvchc_plugin_check_finalize(void *ctx, void *check);

#ifdef __cplusplus
extern "C"
#endif
const dsvchc_plugin_str_t *
dsvchc_plugin_check_get_error(void *ctx, void *check);

#ifdef __cplusplus
extern "C"
#endif
void
dsvchc_plugin_check_destroy(void *ctx, void *check);

#ifdef __cplusplus
extern "C"
#endif
int32_t
dsvchc_plugin_ctx_finalize(void *ctx);

#ifdef __cplusplus
extern "C"
#endif
const dsvchc_plugin_str_t *
dsvchc_plugin_ctx_get_error(void *ctx);

#ifdef __cplusplus
extern "C"
#endif
void
dsvchc_plugin_ctx_destroy(void *ctx);

#endif //DSVCHC_PLUGIN_H
