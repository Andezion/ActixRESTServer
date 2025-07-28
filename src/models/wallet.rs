use bdk::{Wallet}; // импортируем деньги
use bdk::database::MemoryDatabase; // импортируем базу
use bdk::bitcoin::Network; // условно к какой сети относимся
use bdk::keys::{DerivableKey, ExtendedKey, bip39::{Mnemonic}}; // чтобы работать с ключами
use bdk::template::Bip84; // чтобы генерировать адресса но это пиздец


/*
    Это нужно чтобы создать кошелёк.
    У меня не получилось сделать это по модному как в документации так что я просто
    спёр то что увидел на гитхабе, и смешную мнемонику взял.

 */
pub fn create_wallet() -> Wallet<MemoryDatabase> {
    let mnemonic_words = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
    let mnemonic = Mnemonic::parse(mnemonic_words).unwrap(); // тут переделываем

    // Вообще нужно вот так делать но как-то не задалось
    // let mnemonic = Mnemonic::generate(WordCount::Words12).unwrap();

    // как в книжке - наш супер ключ, с которого мы уже можем генерить остальные ключи
    let xkey: ExtendedKey = mnemonic.into_extended_key().unwrap();
    let xprv = xkey.into_xprv(Network::Testnet).unwrap(); // собстенно сгенерировали ключ наш от родителя

    // тут мы создаем наше супер крутое число для кошелька
    let descriptor = Bip84(xprv, bdk::KeychainKind::External);

    // передаём все наши данные
    Wallet::new(
        descriptor, // крутое число которое описывает кошелёк
        None, // тут что-то сложное
        Network::Testnet, // наша сеть
        MemoryDatabase::default(), // дефолтная база
    ).unwrap()
}