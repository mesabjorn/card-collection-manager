# Getting a series as json

1. go to a series page:
e.g. https://yugioh.fandom.com/wiki/Set_Card_Lists:Legend_of_Blue_Eyes_White_Dragon_(TCG-EN)
2. 
```js
function tableToJson(tableId) {
    const table = document.getElementById(tableId);
    const rows = Array.from(table.querySelectorAll("tbody tr"));
    
    return rows.map(row => {
        const cells = row.querySelectorAll("td");
        return {
            card_number: cells[0]?.innerText.trim() || "",
            name: cells[1]?.innerText.replace(/"/g, "").trim() || "",
            rarity: cells[2]?.innerText.trim() || "",
            category: cells[3]?.innerText.trim() || ""
        };
    });
}

const jsonData = tableToJson("Top_table");
console.log(JSON.stringify(jsonData, null, 2));

```
3. Copy the result to a json file.
