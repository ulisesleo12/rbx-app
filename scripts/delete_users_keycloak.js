const fs = require('fs');
const axios = require('axios');

// We read the JSON file
fs.readFile('deleted_users_example.json', 'utf8', (err, data) => {
    if (err) {
        console.error('Error reading file:', err);
        return;
    }

    // We parse the JSON file
    const users = JSON.parse(data);

    // Feature to remove a Keycloak user by their ID
    const deleteUserFromKeycloak = async (userId) => {
        try {
        const url = `https://auth.roboxmaker.com/auth/admin/realms/aker/users/${userId}`;
        
        // Here you must add the authentication token that is used to authorize the request
        const config = {
            headers: {
                // 'Authorization': 'Bearer '
                'Authorization': 'Bearer '
            }
        };

        // We make the DELETE request
        const response = await axios.delete(url, config);

        setTimeout(() => {
            // console.log("Delayed for 1 second.");
        }, "3000")
        
        if (response.status === 204) {
            console.log(`User with ID ${userId} successfully deleted.`);
        } else {
            console.log(`Error deleting user with ID ${userId}.`);
        }
        } catch (error) {
            console.error(`Error when making the request to delete the user with ID ${userId}:`, error.response ? error.response.data : error.message);
        }
    };

    // Loop through all users and delete each one by their ID
    users.forEach(user => {
        const userId = user.id;
        console.log(`Deleting user with ID: ${userId}`);
        deleteUserFromKeycloak(userId);
    });
});