echo off

:: Run the cargo command to build the app
cargo %1 %2

:: Some of the directories we need to work with
set template_dir=%CD%\templates
set debug_dir=%CD%\target\debug\
set release_dir=%CD%\target\release\

echo %template_dir%
echo %debug_dir%
echo %release_dir%

:: If the debug directory exists copy the templates directory there
IF EXIST %debug_dir% (
    :: Make sure to clear out the old templates directory
    IF EXIST %debug_dir%templates (
        rmdir %debug_dir%templates
    )
    mkdir %debug_dir%templates

    :: Copy all of the templates over
    xcopy %template_dir% %debug_dir%templates /y
)

:: If the release directory exists copy the templates directory there
IF EXIST %release_dir% (
    :: Make sure to clear out the old templates directory
    IF EXIST %release_dir%templates (
        rmdir %release_dir%templates
    )
    mkdir %release_dir%templates

    :: Copy all of the templates over
    xcopy %template_dir% %release_dir%templates /y
)