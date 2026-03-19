const { exec } = require("child_process");
const { log } = require("console");

roboxmaker_school= {
	"data": {
		"school_group": {
            "inventory_group": {
                "group_id": "64cd9936-89c3-4e39-b963-e82ae8025fc9"
            },
            "school": {
                "school_profile": {
                    "name": "ROBOXMAKER ACADEMY",
                    "school_id": "f7a0cc38-d35a-496a-a442-6f40fdfe36a5"
                }
            }
        },
		"class_group": [
			// {
			// 	"class_profile": {
			// 		"name": "Kinder 4-A"
			// 	},
			// 	"group_id": "26b7d5ef-58ac-4f08-95b5-db7ff60e636e"
			// },
			{
				"class_profile": {
					"name": "Kinder 5-A"
				},
				"group_id": "1edd9be3-561f-4d08-86cd-15c6843de975"
			},
			{
				"class_profile": {
					"name": "Preparatoria-A"
				},
				"group_id": "77ae25af-2299-4384-b555-3a124a920b87"
			},
			{
				"class_profile": {
					"name": "Primaria 1-A"
				},
				"group_id": "7d92a7f7-d0b2-4a8a-9ecd-c51953ddd9e8"
			},
			{
				"class_profile": {
					"name": "Primaria 2-A"
				},
				"group_id": "73c9e0b1-4cd5-4a1e-993c-5513e5612e35"
			},
			{
				"class_profile": {
					"name": "Primaria 3-A"
				},
				"group_id": "87c95e2f-18d3-49b3-9b85-3c08765725b9"
			},
			{
				"class_profile": {
					"name": "Primaria 4-A"
				},
				"group_id": "55d01bdd-2836-49d6-b80c-a285254ac5a7"
			},
			{
				"class_profile": {
					"name": "Primaria 5-A"
				},
				"group_id": "c58e3cf6-f5ec-4833-a29c-6a1b36e5ee91"
			},
			{
				"class_profile": {
					"name": "Primaria 6-A"
				},
				"group_id": "c6608741-063c-40f2-9e31-67f2a49031d8"
			},
			{
				"class_profile": {
					"name": "Secundaria 1-A"
				},
				"group_id": "9bd0f6a3-8626-4660-851e-7ef52bb83415"
			},
			{
				"class_profile": {
					"name": "Secundaria 2-A"
				},
				"group_id": "1bf7d694-9494-4872-8e21-ef8024a57f7b"
			},
			{
				"class_profile": {
					"name": "Secundaria 3-A"
				},
				"group_id": "5cc682e4-f764-466a-b60d-eb82b3e327e8"
			},
			{
				"class_profile": {
					"name": "Bachillerato 1-A"
				},
				"group_id": "7491dbc5-4e23-4838-95af-16645caec261"
			},
			{
				"class_profile": {
					"name": "Bachillerato 2-A"
				},
				"group_id": "dbe6e31b-423d-4408-aeca-5bc94ac23872"
			}
		]
	}
}

user_id = "b3b440ae-ab59-49eb-ba76-40a68d040862" // Adrian Martinez 22/10/24

roboxmaker_school.data.class_group.forEach(class_group => {
    // console.log(class_group)

    cmd = `npm run aker-gql-op -- group_member_add --group_id ${class_group.group_id} --user_id ${user_id}`;
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
