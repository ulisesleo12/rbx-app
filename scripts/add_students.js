const {exec} = require("child_process");
const { log } = require("console");
const fs = require('fs');

var xlsx = require('node-xlsx');

var data = xlsx.parse(fs.readFileSync(__dirname + '/COLEGIO-CRISTOBAL-COLON.xlsx')); 

// console.log(data[0].data.shift());
data[0].data.shift();

// console.log(data[0]['data']);

function getGroupId(code, section) {
    let group_id = "";
    switch(code) {
        case "K4":
            switch(section) {
                case "A": 
                    group_id = "2a7ccb13-d89b-4987-93d5-7d7669147edf";
                    break;
                case "B": 
                    group_id = "2b4dce57-5b62-42a1-a82f-5d90f30da7eb";
                    break;
                default:
                    group_id = "";
            }
            break;
        case "K5":
            switch(section) {
                case "A": 
                    group_id = "ae8b21df-ec43-4370-9fe6-53b60b889144";
                    break;
                case "B": 
                    group_id = "dd449a1c-4f0b-4779-be95-7932ee67832a";
                    break;
                default:
                    group_id = "";
            }
            break;
        case "PP":
            switch(section) {
                case "A": 
                    group_id = "bff28d59-8d1d-4e40-80b1-f5fca7601468";
                    break;
                case "B": 
                    group_id = "7e377564-f4e3-4f66-bdce-10783c4b5fc7";
                    break;
                default:
                    group_id = "";
            }
            break;
        case "P1":
            switch(section) {
                case "A": 
                    group_id = "2432247b-0c1b-4eb0-a7d7-c0658b440670";
                    break;
                case "B": 
                    group_id = "e9685372-6c7b-4807-9550-3bdcdd4cd7f5";
                    break;
                default:
                    group_id = "";
            }
            break;
        case "P2":
            switch(section) {
                case "A": 
                    group_id = "8db24d62-0b7e-44f1-8ad4-11f3a2c2f5fe";
                    break;
                case "B": 
                    group_id = "641d5f01-f6c8-4da1-80a6-ede4647397a8";
                    break;
                default:
                    group_id = "";
            }
            break;
        case "P3":
            switch(section) {
                case "A": 
                    group_id = "c4a3c5bd-0f06-4afd-8fcf-f24d935c7ac6";
                    break;
                case "B": 
                    group_id = "0514e05c-792b-42cd-b479-5eb0b3e5beca";
                    break;
                default:
                    group_id = "";
            }
            break;
        case "P4":
            switch(section) {
                case "A": 
                    group_id = "8f2918d7-00db-4d79-992a-d6f28bb5373c";
                    break;
                case "B": 
                    group_id = "e9e4de09-0cb9-46cf-9e35-3e914eb3d755";
                    break;
                default:
                    group_id = "";
            }
            break;
        case "P5":
            switch(section) {
                case "A": 
                    group_id = "ff12b194-87a7-4ea2-8dc6-51a559f00ada";
                    break;
                case "B": 
                    group_id = "d3e8f40d-75fe-40e8-a689-efd807b10d93";
                    break;
                default:
                    group_id = "";
            }
            break;
        case "P6":
            switch(section) {
                case "A": 
                    group_id = "11f9c88e-dbd1-437d-92c1-83f8f95be788";
                    break;
                case "B": 
                    group_id = "495a9217-89ce-4b58-9dab-5643c631b23d";
                    break;
                default:
                    group_id = "";
            }
            break;
        case "S1":
            switch(section) {
                case "A": 
                    group_id = "7ba6b3f9-9157-403a-b38e-1a4235e69c9f";
                    break;
                case "B": 
                    group_id = "1b2946eb-0a0a-420a-a9ca-8ff68f0bd541";
                    break;
                default:
                    group_id = "";
            }
            break;
        case "S2":
            switch(section) {
                case "A": 
                    group_id = "107f23ee-8d68-4649-ac06-2551d27a2ae5";
                    break;
                case "B": 
                    group_id = "c4ce908b-698e-4761-bf4d-e9f185aac785";
                    break;
                default:
                    group_id = "";
            }
            break;
        case "S3":
            switch(section) {
                case "A": 
                    group_id = "e3694aa5-9454-4931-9880-d28f7da6b199";
                    break;
                case "B": 
                    group_id = "322ddec0-f726-4d4e-8cac-206a1d765a52";
                    break;
                default:
                    group_id = "";
            }
            break;
        case "B1":
            switch(section) {
                case "A": 
                    group_id = "392ec029-add3-4b64-ab44-a5ac8fb6f7c4";
                    break;
                case "B": 
                    group_id = "b19b1c84-2636-40df-a6f1-f06fba87f096";
                    break;
                case "C": 
                    group_id = "7f29fcde-6dee-46e7-9120-2ddbced6c19e";
                    break;
                default:
                    group_id = "";
            }
            break;
        case "B2":
            switch(section) {
                case "A": 
                    group_id = "c836f443-122c-4eb3-9d9d-5139b484646c";
                    break;
                case "B": 
                    group_id = "0de97987-aecc-4ead-9833-c157baf72a90";
                    break;
                default:
                    group_id = "";
            }
            break;
        default:
            group_id = "";
    }
    return group_id;
}


for (let index = 0; index < 555; index++) {
    let school_id = 'f5c3c010-b381-4835-adbe-890ddb60995f'; // Colegio Cristobal Colon
    let inventory_group_id = '4a43badc-a0ec-49d0-b735-c803143f03cd';
    let group_id = getGroupId(data[0]['data'][index][3], data[0]['data'][index][4]);
    
    let first_name = data[0].data[index][0];
    let last_name = data[0].data[index][1];
    let full_name = first_name + ' ' + last_name;
    let pic_path = 'https://files.roboxmaker.com/uploads/avatar.png';
    let email = data[0].data[index][2];
    let password = data[0].data[index][5];
    let role = 'student';
    let user_id = data[0].data[index][8];
    let license = data[0].data[index][7];
    let username = data[0].data[index][5];

    // console.Console(data[0].data[index][3], data[0].data[index][4])
    // console.log(index, data[0].data[index][2]);
    // console.log(index, data[0].data[index][2], `${data[0]['data'][index][3]} + ${data[0]['data'][index][4]}`, group_id);

    let cmd = `npm run aker-gql-op -- new_user_create --user_id "${user_id}" --license "${license}" --full_name "${full_name}" --pic_path "${pic_path}" --first_name "${first_name}" --last_name "${last_name}" --email "${email}" --password "${password}" --role "${role}" --school_id "${school_id}" --group_id "${group_id}" --inventory_group_id "${inventory_group_id}"  --username "${username}"`;
    console.log(cmd);

    setTimeout(() => {
        // console.log("Delayed for 1 second.");
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
}