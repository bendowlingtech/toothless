//
// Created by ben on 24/11/24.
//

#ifndef PHP_WRAPPER_H
#define PHP_WRAPPER_H

#ifdef __cplusplus
extern "C" {
#endif

void init_php(const char *ini_path, const char **extensions, int ext_count);
void shutdown_php();
char* execute_php_script(const char *script, const char *method, const char *query_string, const char *content);

#ifdef __cplusplus
}
#endif

#endif // PHP_WRAPPER_H

