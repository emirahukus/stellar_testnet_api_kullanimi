Stellar Testnet API Kullanımı
Bu proje, Stellar Testnet API'sini kullanarak çeşitli işlemleri gerçekleştirmek için yazılmış bir Rust uygulamasıdır. Uygulama, Stellar ağı üzerinde hesap bilgilerini çekme, bakiye ve işlem geçmişi sorgulama, ödeme yapma gibi işlemleri destekler.

Özellikler
Hesap Bilgilerini Getirme: Stellar hesabının bilgilerini alır.
Bakiye Sorgulama: Hesap bakiyelerini çekmek için kullanılır.
İşlem Geçmişi: Hesap üzerindeki işlemlerin geçmişini getirir.
Ödeme Oluşturma: Stellar ağı üzerinden ödeme yapar.
Ödeme Geçmişi: Hesap için yapılan ödemelerin geçmişini getirir.
Gereksinimler
Rust: Rust dilini kullanarak kodu çalıştırabilirsiniz.
reqwest ve serde: HTTP istekleri göndermek ve JSON verilerini işlemek için bu kütüphaneler gereklidir.
Kurulum
Proje Bağımlılıklarını Yükleyin: Proje dizininde terminal açın ve bağımlılıkları yüklemek için cargo build komutunu çalıştırın.

Yapılandırma:

Kodda kullanılan Stellar public key (STELLAR_PUBLIC_KEY) ve secret key (STELLAR_SECRET_KEY) testnet ortamına göre ayarlanmıştır.
create_payment fonksiyonunda ödeme yapacağınız destination adresini ve amount değerini değiştirin.
Kullanım
Ana Fonksiyon: Kodun ana işlevi main fonksiyonunda bulunur. Bu fonksiyon sırasıyla aşağıdaki işlemleri yapar:

Hesap bilgilerini alır.
Bakiye bilgilerini sorgular.
İşlem geçmişini alır.
Örnek bir ödeme yapar.
Ödeme geçmişini getirir.
Kodunuzu çalıştırmak için terminalde cargo run komutunu kullanabilirsiniz.

Kod Açıklamaları
fetch_account_info: Hesap bilgilerini alır ve yazdırır.
fetch_balance: Hesap bakiyelerini alır ve yazdırır.
fetch_transactions: Hesap üzerindeki işlemleri alır ve yazdırır.
create_payment: Belirtilen adrese ödeme yapar.
fetch_payment_history: Hesap için yapılan ödemelerin geçmişini alır ve yazdırır.
