# Quick Description
It's a program I quickly threw together because of a unamed parties false claim on some duel statistics
So don't exspect too mutch from it, the code was done quick
# Use Instructions
1. Make sure you're logged into steam on your browser of choice and open: 
[Steam Duel Stats](https://steamcommunity.com/my/gcpd/440?tab=playerduelhistory)
2. Load **all** the information you can do this by clicking show more repetedly or just open up console
Control+Shift   +K (for firefox)    +J(for chrome) Paste in this command: 
```
setInterval(function(){
 if (document.querySelector('#load_more_button').style.display !== 'none'){
  document.querySelector('#load_more_button').click();
 }
},2000);
```
and do something else with your time while it loads!

3. Save the page somewhere.
(Ctrl + S or go source code and copy paste it into a file, anything will work really)
4. Eather compile the program or get it from [Releases](https://github.com/SFort/TF2-Duel_history_formatter/releases)
5. That's it! you can now use the program by running it in console with ./program_name file_path
If your not familliar with console just do ./program_name then drag the html file into the console

Avalable commands :
  - kd
  - result
### Need help
- So far I don't know what the End Reason values actually mean so if somebody does please submit a issue on it
- Obviously there are a lot of code improvments to be made but i would not exspect anybody to bother.
# Output Screenshots
(Ignore the unrecognised character boxes that's just my pc)

K/D Ratio    | Main output
------------ | -------------
![Image of k/d command](https://i.imgur.com/yGNLjM5.png) | ![Image of result command](https://i.imgur.com/vH7f1KP.png)


### Credits
- [Dogman](https://github.com/dogman176) Debuging and compiling the windows version.
