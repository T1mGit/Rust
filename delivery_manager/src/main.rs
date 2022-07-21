use text_io::try_read;

fn main() {
    println!("Hello, world!");

//Declare Menu lists
    let menu_mode_select = ["1)Manage Users", "2)Manage Clients", "3)Export Data"];
    let menu_manage_users = ["1)Add User","2)Delete User","3)Update User"];
    let menu_manage_clients = ["1)New","2)Search"];


    let selection=display_menu(&menu_mode_select);
    println!("You selected {}",selection);

    let selection=display_menu(&menu_manage_users);
    println!("You selected {}",selection);

    let selection=display_menu(&menu_manage_clients);
    println!("You selected {}",selection);
}

//Function 'display_menu' will print lines of text taken from an array of strings
//Strings should include line number so a user my select appropriate number.
//User must enter an integer corresponding to the line number 1 to n for the respective item.
//This function returns the line number as an int32. 
//Any value not a positive integer is rejected and user is asked again.
fn display_menu(menu_items: &[&str])->i32{

    loop {

//Print the menu items
//Wait for user to select number
    for item in menu_items {
	println!("{}",item);
    }
    println!("Please select a valid item number.");

//Get user input as integer. Return invalid menu item (0) for any error to avoid crashing, so that the user will be prompted again.
	
	let n: i32 = match try_read!() {
	    Ok(n)=>n,
	    Err(_error)=>{
		0
	    },
	};
	
//make sure input is valid menu option (numbered from 1 to n) [note 'n' is i32 type by len() is usize and must be converted
	if (n>0) && (n<= menu_items.len().try_into().unwrap()) {
	    return n;
	} else {
	    println!("Not a menu item\n\n\n");
	}

    }
}


