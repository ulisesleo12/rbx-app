const fs = require('fs');
const {exec} = require("child_process");

// We read the JSON file
fs.readFile('deleted_users_example.json', 'utf8', (err, data) => {
    if (err) {
        console.error('Error reading file:', err);
        return;
    }

    // We parse the JSON file
    const users = JSON.parse(data);

    // Loop through all users and delete each one by their ID
    console.log(`Total users to delete: ${users.length}`);

    users.forEach(user => {
        const userId = user.attributes['x-hasura-user-id'][0];
        // console.log(userId);
        // console.log(`Deleting user with ID: ${userId}`);
        cmd = `npm run aker-gql-op -- delete_user --user_id ${userId}`;

        console.log(cmd);

        setTimeout(() => {
            console.log("Delayed for 1 second.");
        }, "3000")

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
});