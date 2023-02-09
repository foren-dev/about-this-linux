package main

import (
	"fmt"
	"fyne.io/fyne/v2"
	"fyne.io/fyne/v2/app"
	"fyne.io/fyne/v2/canvas"
	"fyne.io/fyne/v2/container"
	"fyne.io/fyne/v2/layout"
	"fyne.io/fyne/v2/theme"
	"fyne.io/fyne/v2/widget"
	"github.com/elastic/go-sysinfo/providers/linux"
	"github.com/klauspost/cpuid/v2"
	"image/color"
	"os"
	"os/user"
)

var canvasSize = fyne.Size{Width: 600, Height: 300}
var aboutThisLinux *canvas.Text
var hostName *canvas.Text
var kernel *canvas.Text
var cpu *canvas.Text

var title = "About This Linux"
var a = app.New()
var window fyne.Window
var name, _ = linux.MachineID()
var kernelName, _ = linux.KernelVersion()
var cpuModel = cpuid.CPU.BrandName

var _ fyne.Theme = (*CustomTheme)(nil)

func main() {
	drawCanvas()
}

func initWidgets() {
	getUserInfo()
	aboutThisLinux = canvas.NewText("About This Linux:", color.White)
	hostName = canvas.NewText("Username: "+name, color.White)
	kernel = canvas.NewText("Kernel: "+kernelName, color.White)
	cpu = canvas.NewText("CPU: "+cpuModel, color.White)

	aboutThisLinux.TextSize = 22
	aboutThisLinux.TextStyle = fyne.TextStyle{Bold: true}

	content := container.New(layout.NewVBoxLayout(), aboutThisLinux, widget.NewSeparator(), hostName, kernel, cpu)
	window.SetContent(content)
}

func drawCanvas() {
	window = a.NewWindow(title)
	window.SetFixedSize(true)
	window.Resize(canvasSize)
	a.Settings().SetTheme(&CustomTheme{})
	initWidgets()
	window.ShowAndRun()
}

func getUserInfo() {
	username, err := user.Current()
	if err != nil {
		fmt.Println("Failed to get username!", err)
		os.Exit(1)
	}
	name = username.Username
}

type CustomTheme struct {
}

func (c CustomTheme) Color(fyne.ThemeColorName, fyne.ThemeVariant) color.Color {
	return color.RGBA{R: 25, G: 25, B: 25, A: 15}
}

func (c CustomTheme) Font(style fyne.TextStyle) fyne.Resource {
	return theme.DefaultTheme().Font(style)
}

func (c CustomTheme) Icon(iconName fyne.ThemeIconName) fyne.Resource {
	return theme.DefaultTheme().Icon(iconName)
}

func (c CustomTheme) Size(sizeName fyne.ThemeSizeName) float32 {
	return theme.DefaultTheme().Size(sizeName)
}
