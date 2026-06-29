
// src/models.rs

use serde::{Serialize, Deserialize};
use crate::errors::NikanError;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentMethod {
    Cash,
    Online,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Organization {
    pub id: u32,
    pub name: String,
    pub active: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Donor {
    pub id: u32,
    pub organization_id: u32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub total_donated: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: u32,
    pub donor_id: u32,
    pub organization_id: u32,
    pub amount: f64,
    pub method: PaymentMethod,
}


impl Donor {

    pub fn new(id: u32, og_id: u32, first: &str, last: &str, email: &str) -> Self {
        Self {
            id, 
            organization_id: og_id,
            first_name: first.to_string(),
            last_name: last.to_string(),
            email: email.to_string(),
            total_donated: 0.0,
        }
    }

    pub fn add_donation(&mut self, amount: f64) {
        if amount > 0.0 {
            self.total_donated += amount;
        }
    }

   
    
}


#[derive(Serialize, Deserialize, Debug)]
pub struct NikanApp {
    pub organization: Organization,
    pub donors: Vec<Donor>,
    pub transactions: Vec<Transaction>,
}

impl NikanApp {
    pub fn new(org : Organization) -> Self {
        Self {
            organization: org,
            donors : Vec::new(),
            transactions : Vec::new(),
        }
    }

    pub fn register_donor(&mut self, donor: Donor) {
        self.donors.push(donor);
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), NikanError> {    

        if transaction.amount <= 0.0 {
            return Err(NikanError::InvalidAmount);
        }
        // // استفاده از iter_mut به ما اجازه می‌دهد دیتای داخل وکتور را ویرایش کنیم
        let donor_found = self.donors.iter_mut().find(|d| d.id == transaction.donor_id);

        match donor_found {
            Some(donor) => {
                // ۲. به‌روزرسانی مبلغ اهداکننده
                donor.add_donation(transaction.amount);
                
                // ۳. ذخیره خود تراکنش در لیست
                self.transactions.push(transaction);
                Ok(())

            }
            None => {
                //println!("⚠️ خطا: اهداکننده‌ای با کد {} یافت نشد. تراکنش رد شد.", transaction.donor_id);
                Err(NikanError::DonorNotFound(transaction.donor_id))
            }
            
        }

    }

    pub fn show_report(&self) {
        println!("\n--- 📊 گزارش موسسه {} ---", self.organization.name);
        println!("تعداد کل اهداکنندگان: {}", self.donors.len());
        for donor in &self.donors {
            println!("- {} {}: {} تومان کمک مالی", donor.first_name, donor.last_name, donor.total_donated);
        }
        println!("تعداد کل تراکنش‌های ثبت شده: {}", self.transactions.len());
        println!("--------------------------------");
    }


    // متد ذخیره اطلاعات در فایل JSON
    pub fn save_to_file(&self, file_path: &str) -> Result<(), std::io::Error> {
        // تبدیل کل ساختار برنامه به یک رشته JSON خوانا
        let json_string = serde_json::to_string_pretty(self)?;
        
        // نوشتن رشته در فایل
        std::fs::write(file_path, json_string)?;
        println!("💾 اطلاعات پلتفرم نیکان با موفقیت در فایل ذخیره شد.");
        Ok(())
    }

    // متد بازیابی اطلاعات از فایل JSON
    pub fn load_from_file(file_path: &str) -> Result<Self, std::io::Error> {
        // خواندن کل فایل به صورت رشته
        let json_string = std::fs::read_to_string(file_path)?;
        
        // تبدیل متن JSON به Structهای راست
        let app: Self = serde_json::from_str(&json_string)?;
        println!("📂 اطلاعات قبلی با موفقیت از فایل بارگذاری شد.");
        Ok(app)
    }


    

}





#[cfg(test)] // این اتریبیوت به راست می‌گوید این ماژول را فقط موقع cargo test اجرا کن
mod tests {
    use super::*; // دسترسی به تمام Structها و متدهای ماژول بیرونی (Donor, NikanApp و...)

    #[test] // مشخص می‌کند که این یک تابع تست است
    fn test_add_transaction_success() {
        // ۱. آماده‌سازی داده‌های اولیه (Setup)
        let org = Organization { id: 1, name: "تست".to_string(), active: true };
        let mut app = NikanApp::new(org);
        
        let donor = Donor::new(1, 1, "علی", "احمدی", "ali@test.com");
        app.register_donor(donor);

        let tx = Transaction {
            id: 10,
            donor_id: 1, // این آیدی وجود دارد
            organization_id: 1,
            amount: 5000.0,
            method: PaymentMethod::Cash,
        };

        // ۲. اجرای متد مورد نظر
        let result = app.add_transaction(tx);

        // ۳. بررسی نتیجه (Assertions)
        assert!(result.is_ok()); // مطمئن می‌شویم که خروجی Ok است
        assert_eq!(app.transactions.len(), 1); // مطمئن می‌شویم تراکنش اضافه شده
        assert_eq!(app.donors[0].total_donated, 5000.0); // مطمئن می‌شویم به مجموع کمک‌ها اضافه شده
    }

    #[test]
    fn test_add_transaction_donor_not_found() {
        let org = Organization { id: 1, name: "تست".to_string(), active: true };
        let mut app = NikanApp::new(org); // در این app هیچ اهداکننده‌ای ثبت نمی‌کنیم

        let tx = Transaction {
            id: 20,
            donor_id: 999, // این آیدی وجود ندارد!
            organization_id: 1,
            amount: 2000.0,
            method: PaymentMethod::Online,
        };

        let result = app.add_transaction(tx);

        // بررسی اینکه آیا دقیقاً ارور DonorNotFound با آیدی 999 برگشته یا نه
        // نکته: برای مقایسه مستقیم Enumها باید روی آن‌ها ویژگی PartialEq تعریف شده باشد،
        // اما اینجا با ساختار match یا متدهای دیگر هم می‌توان بررسی کرد:
        assert!(result.is_err());
    }
}