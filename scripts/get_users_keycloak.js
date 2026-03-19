const fs = require('fs');

function filterAndSaveByRole(role, inputFileName, outputFileName) {
    // We read the JSON file
    fs.readFile(inputFileName, 'utf8', (err, data) => {
        if (err) {
            console.error('Error reading file:', err);
            return;
        }

        // We convert the JSON file into a JavaScript object
        const users = JSON.parse(data);

        // We filter the users who have the specified role
        const filteredUsers = users.filter(user => {
            return user.attributes['x-hasura-default-role'] &&
                    user.attributes['x-hasura-default-role'].includes(role);
        });

        // We show the results
        // console.log(`Users with the role "${role}":`, filteredUsers.length);
        console.log(`Users with the role "${role}":`, filteredUsers);


        // We save the filtered result in a new JSON file
        fs.writeFile(outputFileName, JSON.stringify(filteredUsers, null, 2), (err) => {
            if (err) {
            console.error('Error saving file:', err);
            } else {
            console.log(`File saved as ${outputFileName}`);
            }
        });
    });
}

filterAndSaveByRole('example', 'users_keycloak_2024.json', 'filtered_users_example.json');