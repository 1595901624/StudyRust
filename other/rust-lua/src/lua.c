#include "../lua/src/lua.h"
#include "../lua/src/lauxlib.h"
#include "../lua/src/lualib.h"

extern int execute(const char *code);

int execute(const char *code)
{
    lua_State *L = luaL_newstate();

    luaL_openlibs(L);

    if (luaL_dostring(L, code))
    {
        const char *error = lua_tostring(L, -1);

        printf("Error: %s\n", error);

        lua_pop(L, 1);

        lua_close(L);

        return 1;
    }

    lua_close(L);

    return 0;
}