//
// Created by ben on 24/11/24.
//

#ifndef TOOTHLESS_PHP_WRAPPER_H
#define TOOTHLESS_PHP_WRAPPER_H

#include "php_wrapper.h"
#include <php_embed.h>
#include <stdlib.h>
#include <string.h>

void init_php(const char *ini_path, const char **extensions, int ext_count) {
    char *argv[] = { "embedded_php", NULL };
    php_embed_init(1, argv);

    if (ini_path && strlen(ini_path) > 0) {
        zend_alter_ini_entry("user_ini.filename", sizeof("user_ini.filename")-1, ini_path, strlen(ini_path), PHP_INI_SYSTEM, PHP_INI_STAGE_ACTIVATE);
    }

    for (int i = 0; i < ext_count; i++) {
        zend_module_entry *module = (zend_module_entry *)zend_hash_str_find_ptr(&module_registry, extensions[i], strlen(extensions[i]));
        if (module && module->startup) {
            module->startup(module, 0);
        }
    }
}

void shutdown_php() {
    php_embed_shutdown();
}

char* execute_php_script(const char *script, const char *method, const char *query_string, const char *content) {

    SG(server_context) = NULL;
    SG(request_info).request_method = (char*)method;
    SG(request_info).query_string = (char*)query_string;
    SG(request_info).request_uri = "/";
    SG(request_info).path_translated = "";
    SG(request_info).content_type = "";
    SG(request_info).content_length = strlen(content);
    SG(request_info).auth_user = NULL;
    SG(request_info).auth_password = NULL;
    SG(request_info).argv0 = "";
    SG(request_info).argc = 0;
    SG(request_info).argv = NULL;
    SG(sapi_headers).http_response_code = 200;


    if (strcmp(method, "POST") == 0) {
        php_stream *stream = php_stream_temp_new();
        php_stream_write(stream, content, strlen(content));
        SG(request_info).request_body = stream;
    }

    php_request_startup();


    php_output_start_user(NULL, 0, PHP_OUTPUT_HANDLER_STDFLAGS);


    zend_eval_string((char*)script, NULL, "Embedded PHP");


    zend_string *out_buf = php_output_get_contents();
    php_output_end();

    php_request_shutdown(NULL);


    char *output = strdup(ZSTR_VAL(out_buf));
    zend_string_release(out_buf);

    return output;
}



#endif //TOOTHLESS_PHP_WRAPPER_H
