const { exec } = require("child_process");
const fs = require('fs');

var xlsx = require('node-xlsx');

// let school_id = "e1c3c8c9-81f4-465b-b639-c433c3584307";

// var obj = xlsx.parse(__dirname + '/Licencias2022.xlsx'); 

var data = xlsx.parse(fs.readFileSync(__dirname + '/Licencias2024.xlsx')); 


// let ICO = "94df7d74-ce0a-45ba-9cad-05a1cc636bfb";
let CIO = "59ddc028-79c9-4be2-86c9-b1d430995786";
let CBM = "ff04c23e-1a51-4be1-bc63-e986129b1143";
let NSP = "001862ff-8cd1-4867-a3b0-e4290ae5236c";
let IBE = "5201b5a5-0dca-4f19-a56b-5356d39d0477";
let GSM = "fbad49bf-1f7c-4c1b-a9c9-aee4987e5fa9";
// let MSA = "09ac5158-6e7e-4743-9e75-9713d2047e46";
let CSF = "09fcc879-6760-42df-975b-d1bebe6a7fbf";
let CEP = "840a1b44-3eb7-4941-8eb2-046c20269450";
let LBI = "d15d3e13-388e-40af-8182-ffd7187a5cba";
let SCS = "ecd5dcec-8dff-11ee-b9d1-0242ac120002";
let FAT = "09ac5158-6e7e-4743-9e75-9713d2047e46";
let CCC = "f5c3c010-b381-4835-adbe-890ddb60995f";
let MCA = "658a4461-192a-4dd1-ae0d-1caa9800ed55";
let CFR = "2382e193-c4fa-4120-8615-23fb9d980514";

function getSchoolId(code) {
    // console.log(code);
    let school_id = "";
    switch(code) {
        case "CIO":
            school_id = "59ddc028-79c9-4be2-86c9-b1d430995786";
            break;
        case "CBM":
            school_id = "ff04c23e-1a51-4be1-bc63-e986129b1143";
            break;
        case "NSP":
            school_id = "001862ff-8cd1-4867-a3b0-e4290ae5236c";
            break;
        case "IBE":
            school_id = "5201b5a5-0dca-4f19-a56b-5356d39d0477";
            break;
        case "GSM":
            school_id = "fbad49bf-1f7c-4c1b-a9c9-aee4987e5fa9";
            break;
        case "CSF":
            school_id = "09fcc879-6760-42df-975b-d1bebe6a7fbf";
            break;
        case "CEP":
            school_id = "840a1b44-3eb7-4941-8eb2-046c20269450";
            break;
        case "LBI":
            school_id = "d15d3e13-388e-40af-8182-ffd7187a5cba";
            break;
        case "SCS":
            school_id = "ecd5dcec-8dff-11ee-b9d1-0242ac120002";
            break;
        case "FAT":
            school_id = "09ac5158-6e7e-4743-9e75-9713d2047e46";
            break;
        case "CCC":
            school_id = "f5c3c010-b381-4835-adbe-890ddb60995f";
            break;
        case "MCA":
            school_id = "658a4461-192a-4dd1-ae0d-1caa9800ed55";
            break;
        case "CFR":
            school_id = "2382e193-c4fa-4120-8615-23fb9d980514";
            break;
        default:
            school_id = "";
      }
    return school_id;
}

// 2815 + 3 
// 54
// 15

data[1].data.shift();


let licenses = data[1].data;

// console.log(licenses);
console.log(licenses.length);

// 5740

// CSC 240 -975
// CSC 239 -974


for (let index = 239; index < 269; index++) {
    let school_id = getSchoolId(licenses[index][0]);
    let license = licenses[index][2];

    let cmd = `npm run aker-gql-op -- license_add  --license "${license}" --school_id "${school_id}"`;
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
    // });
}

