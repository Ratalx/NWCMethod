function addSupplier() {
    var suppliers = document.getElementsByClassName("names");
    var supplier = document.createElement('div');
    var prop = document.createElement('div');
    var rows = document.getElementsByClassName("row");
    var cell_prop = document.createElement('div');
    cell_prop.className = "cell_prop";
    cell_prop.innerHTML ="<input type=\"text\" size=\"1\">"
    prop.className = "supplier_prop";
    prop.innerHTML = "<input type=\"text\" size=\"1\"> <input type=\"text\" size=\"1\">"; 
    supplier.className = "cell";
    supplier.innerHTML = 'S' + suppliers[0].children.length;
    supplier.appendChild(prop.cloneNode(true));

    suppliers[0].appendChild(supplier);

    for( i = 0; i < rows.length;++i){
        var cell = document.createElement('div');
        cell.className="cell";
        cell.appendChild(cell_prop.cloneNode(true));
        rows[i].appendChild(cell);
    }
}

function addRecipient() {
    var rows = document.getElementsByClassName("row");
    var row = document.createElement('div');
    var recipient = document.createElement('div');
    var prop = document.createElement('div');
    var cell_prop = document.createElement('div');
    var suppliers = document.getElementsByClassName("names");
    var table = document.getElementsByClassName("table");
    cell_prop.className = "cell_prop";
    row.className = "row";
    prop.className = "recipient_prop";
    prop.innerHTML = "<input type=\"text\" size=\"1\"> <input type=\"text\" size=\"1\">"; 
    cell_prop.innerHTML = "<input type=\"text\" size=\"1\">";
    recipient.className = "cell"
    recipient.innerHTML = 'R' + rows.length;
    recipient.appendChild(prop.cloneNode(true));
    row.appendChild(recipient);
    console.log(suppliers[0].children.length)
    for( i=1; i < suppliers[0].children.length;++i){
        var cell = document.createElement('div');
        cell.className="cell";
        cell.appendChild(cell_prop.cloneNode(true));
        row.appendChild(cell);
    }

    table[0].appendChild(row);
}

function start() {

    var suppliers = document.getElementsByClassName("names");
    var recipients = document.getElementsByClassName("row");
    var json = '{ "suppliers": [';
    for( i=1; i<suppliers[0].children.length; ++i) {
        json += '{\n  "id":' + (i-1) + ',\n"value":' + suppliers[0].children[i].children[0].children[0].value + ',\n"price":' + suppliers[0].children[i].children[0].children[1].value + '\n}';
        if (i < suppliers[0].children.length-1) {
            json += ',\n';
        }
     
    }
    json += '],\n';

    json += '"recipients": [';
    for( i = 0; i<recipients.length;++i) {
        json += '{\n "id":' + i + ',\n"value":' + recipients[i].children[0].children[0].children[0].value + ',\n"price":' + recipients[i].children[0].children[0].children[1].value + '\n}';
        if (i < recipients.length-1) {
            json += ',\n';
        }
    }
    json += '],\n';

    json += '"fields" : ['
    for ( i = 0; i<recipients.length; ++i) {
        console.log(recipients[i].children[1].children[0]);
        for( j = 1; j< recipients[i].children.length;++j) {
            json += '{\n "supplier_id":' + (j-1) + ',\n"recipient_id":' + (i) + ',\n"cost":' + recipients[i].children[j].children[0].children[0].value + ',\n"value":0' + '\n}';
        }
        if(j<recipients[i].children.length -1) {
            json += ',\n';
        }
    }
    json +=']}';
    console.log(json);
    var obj = JSON.parse(json);
    console.log(obj);
}