const {exec} = require("child_process");
const fs = require('fs');

/* 
    Inventory group id 
    LSA: "72d0daf5-14f2-4f31-a6e2-0b2d50193fd6"
    CCC: "4a43badc-a0ec-49d0-b735-c803143f03cd"
    ICO: "cfd9058b-216f-4185-8c64-aff99d752b47"
    MSA: "6dbf9479-53d6-4e79-b6e5-b4afbf2d8a31"
    CIO: "961569ee-98b2-4ab4-a67d-22bcc5114323"
    LEJ: "a14d7d2a-3e4d-4aaf-bce5-9cecd8192897"
    VSA: "f1a350e6-5f43-455b-a1ce-85dafafa38eb"
*/
let group_id = "64cd9936-89c3-4e39-b963-e82ae8025fc9";
let post_id = [['1033cb69-b3cd-4750-8d18-56f8f6b1fb60', 'Ayuda - Elementos mecánicos kínder'], ['4595449b-ab90-4d27-ac06-bc136b99893b', 'Ayuda - Función de ruedas y llantas'], ['355b041d-d477-4769-a894-db02411e4ead', 'Ayuda - Función de arandelas y esferas'], ['59bd833b-e1df-417e-bdae-9a2b4b00b4f3', 'Ayuda - Controles del menú de Roboxmaker'], ['2e3a391e-c3bd-4327-91a9-fb91ca4cc76f', 'Ayuda - Función de conector clip gris / conector clip 180°'], ['c543582a-6b8e-475d-8cb1-f3700294ad30', 'Ayuda - Función de conector clip azul / conector clip 360°'], ['8d94e8ac-f4e1-4d3d-855c-a62e22310324', 'Ayuda - Función de conector naranja / conector de 0° y 180°'], ['5e65ee49-1400-4cc2-9b33-a1798d0368dd', 'Ayuda - Función de conector gris / conector 45°'], ['6b5c7b95-e66c-4fbf-b80f-e127ec9f4901', 'Ayuda - Función de conector rojo / conector 90°'], ['1da74073-693e-428d-8bde-4e5396431930', 'Ayuda - Función de conector verde / conector 135°'], ['87b02722-eb13-421f-8260-d9f76e6c8a69', 'Ayuda - Función de conector amarillo / conector 180°'], ['c88d4caa-3c5c-450c-9621-87b8dc379efb', 'Ayuda - Función de conector blanco / conector 360°'], ['728098f7-5941-486c-851c-bbe049901eae', 'Ayuda - Conectores y ejes'], ['dc16b0b8-5f58-4fea-a2ea-5dfef295ff39', 'Ayuda - Elementos mecánicos']]
        

post_id.forEach(element => {
    let cmd = `npm run aker-gql-op -- post_group_add  --post_id ${element[0]} --group_id ${group_id}`;
    console.log(cmd);

    exec(cmd, (error, stdout, stderr) => {
        if (error) {
            console.log(`error: ${error.message}`);
            return;
        }
        if (stderr) {
            console.log(`stderr: ${stderr}`);
            return;
        }
        console.log(`stdout: ${stdout}`);
    });
});