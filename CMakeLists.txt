cmake_minimum_required(VERSION 3.9)
cmake_policy(SET CMP0042 NEW)
set (CMAKE_CXX_STANDARD 11)

# used to make nvim able to intercept linked directories
# ie. header files will be `not found` without compile_commands.json
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

project (node-sdl)
include_directories(${CMAKE_JS_INC})
file(GLOB SOURCE_FILES
  "src/sdl.cc"
  "src/video.cc"
  "src/events.cc"
)
add_library(${PROJECT_NAME} SHARED ${SOURCE_FILES} ${CMAKE_JS_SRC})
set_target_properties(${PROJECT_NAME} PROPERTIES PREFIX "" SUFFIX ".node")
target_link_libraries(${PROJECT_NAME} ${CMAKE_JS_LIB})

# Include Node-API wrappers
execute_process(COMMAND node -p "require('node-addon-api').include"
        WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
        OUTPUT_VARIABLE NODE_ADDON_API_DIR
        )
string(REGEX REPLACE "[\r\n\"]" "" NODE_ADDON_API_DIR ${NODE_ADDON_API_DIR})

target_include_directories(${PROJECT_NAME} PRIVATE ${NODE_ADDON_API_DIR})

# define NAPI_VERSION
add_definitions(-DNAPI_VERSION=3)

# SDL stuffs
execute_process(COMMAND sdl2-config --libs
        WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
        OUTPUT_VARIABLE SDL_LIBS
        )
string(REGEX REPLACE "[\r\n\"]" "" SDL_LIBS ${SDL_LIBS})
target_link_libraries(${PROJECT_NAME} ${SDL_LIBS})

execute_process(COMMAND sdl2-config --cflags
        WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
        OUTPUT_VARIABLE SDL_FLAGS
        )
string(REGEX REPLACE "[\r\n\"]" "" SDL_FLAGS ${SDL_FLAGS})

target_include_directories(${PROJECT_NAME} PRIVATE ${NODE_ADDON_API_DIR})
set(CMAKE_C_FLAGS ${SDL_FLAGS})
set(CMAKE_CXX_FLAGS ${SDL_FLAGS})


