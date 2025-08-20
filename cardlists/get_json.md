# Getting a series as json

1. go to a series page:
e.g. https://yugioh.fandom.com/wiki/Legend_of_Blue_Eyes_White_Dragon or
 https://yugioh.fandom.com/wiki/Metal_Raiders
2. Run
```js

function filterRarity(text){
    //replace short prints with common
    if (text.indexOf("Short Print")>-1){
        return "Common";
    }
    return text.trim();
}

function tableToJson(tableId) {
    const table = document.querySelectorAll("table.card-list")[0];
    const rows = Array.from(table.querySelectorAll("tbody tr"));
    
    return rows.map(row => {
        const cells = row.querySelectorAll("td");
        return {
            card_number: cells[0]?.innerText.trim() || "",
            name: cells[1]?.innerText.replace(/"/g, "").trim() || "",
            rarity: cells[2]? filterRarity(cells[2].innerText):"",
            category: cells[3]?.innerText.trim() || ""
        };
    });
}

//get card list
const cards = tableToJson();

//get metadata of set
const jsonData = {
    "name":document.querySelector("#mw-content-text > div > aside > h2").innerText,
    "ncards":cards.length,
    "release_date":document.querySelector("#mw-content-text > div > aside > section:nth-child(6) > div:nth-child(2) > div").innerText,
    "cards":cards
}

//dump json in console for copying
console.log(JSON.stringify(jsonData, null, 2));
```
3. Copy to a new json file.

