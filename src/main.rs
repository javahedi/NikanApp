
// src/main.rs

// ۱. معرفی ماژول‌ها به کامپایلر راست
mod errors;
mod models;

// ۲. آوردن ابزارهای مورد نیاز به Scope فعلی برای استفاده راحت‌تر
//use errors::NikanError;
use models::{Organization, NikanApp, Donor, Transaction, PaymentMethod};
use errors::NikanError;



fn handle_transaction_result(result : Result<(), NikanError>) {

    match result {
        Ok(_) => println!("✅ تراکنش با موفقیت در سیستم ثبت نهایی شد."),
        Err(e) => match e {
            NikanError::DonorNotFound(id) => {
                println!("❌ خطای سیستمی: کاربر {} وجود ندارد.", id)
            }
            NikanError::InvalidAmount => {
                println!("❌ خطای سیستمی: مبلغ وارد شده معتبر نیست.")
            }
        },
    }

}


fn main() {

    // Struct Literal :: یعنی شما به طور مستقیم نام تک‌تک فیلدها را می‌آورید و جلویشان مقدار می‌گذارید. 
    let org1 = Organization { id: 1, name: String::from("HelpDunya_Bremen"), active: true, };

    let mut app = NikanApp::new(org1); 
    // Encapsulation 
    let donor1: Donor = Donor::new(101, app.organization.id, "Jim", "Vahedi", "jim@example.com");
    let donor2 = Donor::new(102, app.organization.id, "Javad", "Vahedi", "javahedi@example.com"); 

    let donor1_id = donor1.id;
    let donor2_id = donor2.id;

    app.register_donor(donor1);
    app.register_donor(donor2);
   

    app.show_report();


    let tran1 = Transaction { id: 5001, 
        donor_id: donor1_id, 
        organization_id: app.organization.id,
        amount: 550., 
        method:PaymentMethod::Online, };
    let tran2 = Transaction { id: 5002, 
        donor_id: donor2_id, 
        organization_id: app.organization.id, 
        amount: 376.0, 
        method:PaymentMethod::Online, };
    let tran3 = Transaction { id: 5003, 
        donor_id: donor1_id, 
        organization_id: app.organization.id, 
        amount: 243., 
        method:PaymentMethod::Cash, };


    handle_transaction_result(app.add_transaction(tran1));
    handle_transaction_result(app.add_transaction(tran2));
    handle_transaction_result(app.add_transaction(tran3));
    


    app.show_report();



}