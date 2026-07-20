--// generated from chatgpt, may not be 100% accurate

// WINDOW / CORE

SetTargetFPS()
GetFPS()
GetFrameTime()
GetTime()

GetScreenWidth()
GetScreenHeight()
GetRenderWidth()
GetRenderHeight()

IsWindowFullscreen()

HideCursor()
ShowCursor()
IsCursorHidden()

EnableCursor()
DisableCursor()


// DRAWING

ClearBackground()

DrawPixel()
DrawPixelV()

DrawLine()
DrawLineV()
DrawLineEx()
DrawLineStrip()

DrawCircle()
DrawCircleV()
DrawCircleLines()

DrawEllipse()
DrawEllipseLines()

DrawRectangle()
DrawRectangleV()
DrawRectangleRec()
DrawRectanglePro()
DrawRectangleLines()
DrawRectangleLinesEx()

DrawRectangleGradientV()
DrawRectangleGradientH()
DrawRectangleGradientEx()

DrawRectangleRounded()
DrawRectangleRoundedLines()

DrawTriangle()
DrawTriangleLines()

DrawPoly()
DrawPolyLines()

DrawGrid()


// CAMERA

GetScreenToWorld2D()
GetWorldToScreen2D()

GetCameraMatrix2D()

SetCameraMode()
UpdateCamera()


// TEXTURES / SPRITES

LoadTexture()
LoadTextureFromImage()

UnloadTexture()

IsTextureReady()

UpdateTexture()
UpdateTextureRec()

GetTextureData()

GetTextureSize()

SetTextureFilter()
SetTextureWrap()

DrawTexture()
DrawTextureV()
DrawTextureEx()
DrawTextureRec()
DrawTexturePro()

DrawTextureNPatch()

NPatchInfo

NPATCH_NINE_PATCH
NPATCH_THREE_PATCH_VERTICAL
NPATCH_THREE_PATCH_HORIZONTAL


// IMAGES

Image

LoadImage()
LoadImageRaw()
LoadImageAnim()

UnloadImage()

ExportImage()
ExportImageAsCode()

ImageCopy()
ImageFromImage()

ImageCrop()

ImageResize()
ImageResizeNN()

ImageFormat()

ImageFlipVertical()
ImageFlipHorizontal()

ImageRotate()
ImageRotateCW()
ImageRotateCCW()

ImageColorTint()
ImageColorInvert()
ImageColorGrayscale()
ImageColorContrast()
ImageColorBrightness()

GetImageColor()


// FONTS / TEXT

Font

LoadFont()
LoadFontEx()
LoadFontFromImage()

UnloadFont()

IsFontReady()

DrawText()
DrawTextEx()
DrawTextPro()

MeasureText()
MeasureTextEx()

GetGlyphIndex()
GetGlyphInfo()
GetGlyphAtlasRec()


// INPUT KEYBOARD

IsKeyDown()
IsKeyPressed()
IsKeyReleased()

GetKeyPressed()
GetCharPressed()

SetExitKey()


// INPUT MOUSE

GetMousePosition()
GetMouseDelta()

GetMouseWheelMove()
GetMouseWheelMoveV()

SetMousePosition()
SetMouseOffset()
SetMouseScale()

IsMouseButtonDown()
IsMouseButtonPressed()
IsMouseButtonReleased()


// RECTANGLE

Rectangle

GetCollisionRec()

RectangleContains()
RectangleEquals()


// COLOR

Color

Fade()

ColorToInt()

GetColor()

ColorFromHSV()

ColorToHSV()

ColorLerp()


// COLLISION

CheckCollisionRecs()

CheckCollisionCircles()

CheckCollisionCircleRec()

CheckCollisionPointRec()

CheckCollisionPointCircle()

CheckCollisionPointTriangle()

GetCollisionRec()


// CAMERA / WORLD HELPERS

GetWorldToScreen2D()

GetScreenToWorld2D()


// RENDER TEXTURES

RenderTexture2D

LoadRenderTexture()

UnloadRenderTexture()


// SHADERS

Shader

LoadShader()

LoadShaderFromMemory()

UnloadShader()

GetShaderLocation()

GetShaderLocationAttrib()

SetShaderValue()

SetShaderValueV()

SetShaderValueMatrix()

SetShaderValueTexture()


// AUDIO

IsAudioDeviceReady()

SetMasterVolume()


// SOUND

Sound

LoadSound()

UnloadSound()

PlaySound()

StopSound()

PauseSound()

ResumeSound()

IsSoundPlaying()

SetSoundVolume()

SetSoundPitch()

SetSoundPan()


// MUSIC

Music

LoadMusicStream()

UnloadMusicStream()

PlayMusicStream()

UpdateMusicStream()

StopMusicStream()

PauseMusicStream()

ResumeMusicStream()

IsMusicStreamPlaying()

SetMusicVolume()

SetMusicPitch()


// FILES

FileExists()

DirectoryExists()

GetFileLength()

LoadFileData()

UnloadFileData()

SaveFileData()

LoadFileText()

UnloadFileText()

SaveFileText()

GetFileExtension()

GetFileName()

GetFileNameWithoutExt()


// DEBUG

TraceLog()

SetTraceLogLevel()

DrawFPS()

TakeScreenshot()

SetTraceLogCallback()



// UI PRIMITIVES

DrawTextureNPatch()

DrawTexturePro()

DrawRectangleRounded()

DrawRectangleRoundedLines()

MeasureTextEx()

CheckCollisionPointRec()

GetMousePosition()


// AUTOMATION / RECORDING

AutomationEvent

LoadAutomationEventList()

UnloadAutomationEventList()

ExportAutomationEventList()

SetAutomationEventList()

StartAutomationEventRecording()

StopAutomationEventRecording()

PlayAutomationEvent()


// LOGGING / SYSTEM

SetTraceLogLevel()

SetLoadFileDataCallback()

SetSaveFileDataCallback()

SetLoadFileTextCallback()

SetSaveFileTextCallback()


// SCREENSHOT / CLIPBOARD

TakeScreenshot()

SetClipboardText()

GetClipboardText()


// CURSOR

SetMouseCursor()
