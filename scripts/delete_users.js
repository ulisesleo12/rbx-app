const {exec} = require("child_process");
const fs = require('fs');

fs.readFile('kk_users.json', 'utf8', function (err, data) {

    if (err) {
        console.error('Error al leer el archivo:', err);
        return;
    }

    try {
        // Analiza el contenido del archivo como JSON
        const contenidoJSON = JSON.parse(data);

        console.log(contenidoJSON.length);
            
        contenidoJSON.forEach(user => {
            // let role = user.attributes['x-hasura-default-role'];
            let hasura_user_id = user.attributes['x-hasura-user-id'];

            let cmd = `npm run aker-gql-op -- user_delete_bypk --user_id "${hasura_user_id}"`;

            console.log(cmd);

            // exec(cmd, (error, stdout, stderr) => {
            //     if (error) {
            //         console.log(`error: ${error.message}`);
            //         return;
            //     }
            //     if (stderr) {
            //         console.log(`stderr: ${stderr}`);
            //         return;
            //     }
            //     console.log(`stdout: ${stdout}`);
                
            //     setTimeout(() => {}, 3000 );
            // });

        });

    } catch (error) {
        console.error('Error al analizar el contenido JSON:', error);
    }

  });