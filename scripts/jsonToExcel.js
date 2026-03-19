const fs = require('fs');
const XLSX = require('xlsx');

// Function to convert a JSON file to an Excel file
function jsonToExcel(inputFileName, outputFileName) {
    // We read the JSON file
    fs.readFile(inputFileName, 'utf8', (err, data) => {
        if (err) {
            console.error('Error al leer el archivo:', err);
            return;
        }

        //Convert the JSON file into a JavaScript object
        const users = JSON.parse(data);

        // We prepare the data that we are going to write in Excel
        const excelData = users.map(user => ({
            username: user.username,
            firstName: user.firstName,
            lastName: user.lastName,
            email: user.email,
            role: user.attributes['x-hasura-default-role'] ? user.attributes['x-hasura-default-role'].join(', ') : ''
        }));

        // We create a worksheet from the data
        const ws = XLSX.utils.json_to_sheet(excelData);

        // We create a workbook
        const wb = XLSX.utils.book_new();
        XLSX.utils.book_append_sheet(wb, ws, 'Usuarios');

        // We save the Excel file
        XLSX.writeFile(wb, outputFileName);

        console.log(`Excel file saved as ${outputFileName}`);
    });
}

// We call the function to convert the data from the JSON file to Excel
jsonToExcel('filtered_users_example.json', 'filtered_users_example.xlsx');
