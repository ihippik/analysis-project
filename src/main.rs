// Пусть есть логи:
// System(requestid):
// - trace
// - error
// App(requestid):
// - trace
// - error
// - journal (человекочитаемая сводка)

// Есть прототип штуки, которая умеет:
// - парсить логи
// - фильтровать
//  -- по requestid
//  -- по ошибкам
//  -- по изменению счёта (купить/продать)

// Модель данных:
// - Пользователь (userid, имя)
// - Вещи
//  -- Предмет (assetid, название)
//  -- Набор (assetid, количество)
//      comment{-- Собственность (assetid, userid владельца, количество)}
//  -- Таблица предложения (assetid на assetid, userid продавца)
//  -- Таблица спроса (assetid на assetid, userid покупателя)
// - Операция App
//  -- Journal
//   --- Создать пользователя userid с уставным капиталом от 10usd и выше
//   --- Удалить пользователя
//   --- Зарегистрировать assetid с ликвидностью от 50usd
//   --- Удалить assetid (весь asset должен принадлежать пользователю)
//   --- Внести usd для userid (usd (aka доллар сша) - это тип asset)
//   --- Вывести usd для userid
//   --- Купить asset
//   --- Продать asset
//  -- Trace
//   --- Соединить с биржей
//   --- Получить данные с биржи
//   --- Локальная проверка корректности (упреждение ошибок в ответе)
//   --- Отправить запрос в биржу
//   --- Получить ответ от биржи
//  -- Error
//   --- нет asset
//   --- системная ошибка
// - Операция System
//  -- Trace
//   --- Отправить запрос
//   --- Получить ответ
//  -- Error
//   --- нет сети
//   --- отказано в доступе
use clap::Parser;
use analysis::ReadMode;

#[derive(Parser)]
struct Args {
    /// Path to a log file
    /// #[arg(long)]
    file: String,

    /// Mode: all | errors | exchanges
    #[arg(value_enum, default_value_t = ReadMode::All)]
    mode: ReadMode,
}

fn main() {
    let args = Args::parse();

    let parsing_demo =
        r#"[UserBackets{"user_id":"Bob","backets":[Backet{"asset_id":"milk","count":3,},],},]"#
            .to_string();
    let (_, announcements) =
        analysis::parse::just_parse::<analysis::parse::Announcements>(&parsing_demo).unwrap();
    
    println!("demo-parsed: {:?}", announcements);

    let filename = args.file;

    println!(
        "Trying opening file '{}' from directory '{}'",
        filename,
        std::env::current_dir().unwrap().to_string_lossy()
    );

    let mut file = std::fs::File::open(filename).unwrap();

    let logs = analysis::read_log(&mut file, args.mode, &[]);
    println!("got logs:");
    logs.iter().for_each(|parsed| println!("  {:?}", parsed));
}
