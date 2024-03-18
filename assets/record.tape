# Create euchre-tui demo gif and screenshots
#  VHS syntax reference: https://github.com/charmbracelet/vhs?tab=readme-ov-file#vhs-command-reference

Output assets/demo.gif

Require euchre

Set Shell "zsh"
Set FontSize 16
Set FontFamily "Anonymous Pro"
Set Width 1200
Set Height 800
Set PlaybackSpeed 0.5
Set TypingSpeed 100ms

# run the binary and wait on splash screen
Type "euchre" Sleep 500ms Enter Sleep 3s

Screenshot assets/splash.png

# navigate to setup screen and wait
Type "n" Sleep 2s

# type into first input box and wait
Type "boldandbrad" Sleep 1s Enter Sleep 1s

# type into second input box and wait
Type "Good guys" Sleep 1s Enter Sleep 1s

Screenshot assets/setup.png

# submit form
Enter

# watch game begin
Sleep 10s

Screenshot assets/table.png
