const fs = require('fs');

// Function to convert a JSON file to an Excel file
function jsonToJson(inputFileName, outputFileName) {
    // We read the JSON file
    fs.readFile(inputFileName, 'utf8', (err, data) => {
        if (err) {
            console.error('Error al leer el archivo:', err);
            return;
        }

        //Convert the JSON file into a JavaScript object
        const users = JSON.parse(data);

        // We prepare the data that we are going to write in Excel
        const usersData = users.map(user => ({
            id: user.id,
            username: user.username,
            fullName: user.firstName + ' ' + user.lastName,
            email: user.email,
            role: user.attributes['x-hasura-default-role'] ? user.attributes['x-hasura-default-role'].join(', ') : '',
            user_id: user.attributes['x-hasura-user-id'] ? user.attributes['x-hasura-user-id'].join(', ') : '',
            shool_id: user.attributes['x-hasura-school-id'] ? user.attributes['x-hasura-school-id'].join(', ') : '',
        }));

        // console.log(`Users:`, usersData);

        // We save the filtered result in a new JSON file
        fs.writeFile(outputFileName, JSON.stringify(usersData, null, 2), (err) => {
            if (err) {
                console.error('Error saving file:', err);
            } else {
                console.log(`File saved as ${outputFileName}`);
            }
        });
    });
}

// We call the function to convert the data from the JSON file to Excel
jsonToJson('filtered_users_example.json', 'deleted_users_example.json');
