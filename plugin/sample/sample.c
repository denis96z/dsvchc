#include <stdio.h>
#include <errno.h>
#include <stdlib.h>
#include <string.h>

#include <dsvchc-plugin.h>

#define dsvchc_plugin_sample_panic(format, ...) \
    ({ fprintf(stderr, format " [%s:%d]", ##__VA_ARGS__, __FILE__, __LINE__); exit(1); })

static void *
dsvchc_plugin_sample_xcalloc(size_t count, size_t size);

typedef struct {}
dsvchc_plugin_ctx_t;

void *
dsvchc_plugin_ctx_create()
{
    return dsvchc_plugin_sample_xcalloc(1, sizeof(dsvchc_plugin_ctx_t));
}

int32_t
dsvchc_plugin_ctx_initialize(void *ctx,
                             const dsvchc_plugin_str_t *conf_path)
{
    return DSVCHC_PLUGIN_ERR_CODE_OK;
}

typedef struct {}
dsvchc_plugin_check_t;

void *
dsvchc_plugin_check_create(void *ctx)
{
    return dsvchc_plugin_sample_xcalloc(1, sizeof(dsvchc_plugin_check_t));
}

int32_t
dsvchc_plugin_check_initialize(void *ctx, void *check,
                               const dsvchc_plugin_str_t *conf_path)
{
    return DSVCHC_PLUGIN_ERR_CODE_OK;
}

int32_t
dsvchc_plugin_check_perform(void *ctx, void *check)
{
    return DSVCHC_PLUGIN_ERR_CODE_OK;
}

static const dsvchc_plugin_str_t
dsvchc_plugin_check_result_json = dsvchc_plugin_const_str("{}");

const dsvchc_plugin_str_t *
dsvchc_plugin_check_get_result_json(void *ctx, void *check)
{
    return &dsvchc_plugin_check_result_json;
}

int32_t
dsvchc_plugin_check_finalize(void *ctx, void *check)
{
    return DSVCHC_PLUGIN_ERR_CODE_OK;
}

const dsvchc_plugin_str_t *
dsvchc_plugin_check_get_error(void *ctx, void *check)
{
    dsvchc_plugin_sample_panic("unreachable!");
}

void
dsvchc_plugin_check_destroy(void *ctx, void *check)
{
    free(check);
}

int32_t
dsvchc_plugin_ctx_finalize(void *ctx)
{
    return DSVCHC_PLUGIN_ERR_CODE_OK;
}

const dsvchc_plugin_str_t *
dsvchc_plugin_ctx_get_error(void *ctx)
{
    dsvchc_plugin_sample_panic("unreachable!");
}

void
dsvchc_plugin_ctx_destroy(void *ctx)
{
    free(ctx);
}

static void *
dsvchc_plugin_sample_xcalloc(size_t count, size_t size)
{
    void *x = calloc(count, size);
    if (!x)
        dsvchc_plugin_sample_panic("calloc() failed [count = %zu, size = %zu]: %s", count, size, strerror(errno));
    return x;
}
