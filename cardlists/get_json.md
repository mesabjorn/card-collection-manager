# Getting a series as json

1. go to a series page (e.g.):

   - https://yugioh.fandom.com/wiki/Legend_of_Blue_Eyes_White_Dragon or
   - https://yugioh.fandom.com/wiki/Metal_Raiders
   - or use `.\card-collection-manager.exe .\cards.db find serie "metal raiders"` (the get_series.js will be in your clipboard automatically)

2. Run the get_series.js in the console.
3. Copy to a new json file:

```ps
Get-Clipboard | out-file -encoding ascii <filename>.json
```
