# Open Steam Auth

An gui program that tries to do the same as [steamguard-cli] and [steamDesktopAuthenticator] for now it can only display auth tokens. 


# Disclaimer
This program is not in an useable state at all. For example you cant add accounts or import them. Windows is not supported yet (or at least not tested). Accounts have to be encrypted with [steamDesktopAuthenticator] or [steamguard-cli] (i have had some problems with steamguard-cli but that might be only me. you can run SteamDesktopAuthenticator with proton-call though) 

Also please keep backups of your mafiles. Since this program is still being worked on things could go wrong. I will not be held responsible for any data loss

this program is mainly made for linux users if you use windows use [steamguard-cli]



# using open steam auth
to get the mafiles you can use [steamDesktopAuthenticator] or [steamguard-cli] or extract them from your rooted phone (if you have the old app the new one does not work for now)

and put them in/home/{USER}/.config/steamguard-cli/maFiles



# what i plan to implement

    - [x] reading mafiles and generating codes with them
    - [ ] adding/removing accounts from within the application
    - [ ] accepting/denying trade requests
    - [ ] setting up steam guard for an account within the application

# credits 
 - [steamguard-cli] i used some code of its repo mainly to interact with the mafiles and to interact with steam
 - [tauri] for the gui
 
[steamguard-cli]: https://github.com/dyc3/steamguard-cli
[steamDesktopAuthenticator]: https://github.com/Jessecar96/SteamDesktopAuthenticator
[tauri]: https://tauri.app