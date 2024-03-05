use rand::Rng; // Import the Rng trait to use random number generation

fn get_random_topic<'a>(topics: &'a [&'a str]) -> &'a str {
    let mut rng = rand::thread_rng(); // Create a random number generator.
    let index = rng.gen_range(0..topics.len()); // Generate a random index within the bounds of the topics array.
    topics[index] // Return the topic at the generated random index.
}

fn main() {
    let bible: [&'static str; 66] = [
    // Old Testament
    
    // Pentateuch
    "Genesis",
    "Exodus",
    "Leviticus",
    "Numbers",
    "Deuteronomy",
    
    // Historical Books
    "Joshua",
    "Judges",
    "Ruth",
    "1 Samuel",
    "2 Samuel",
    "1 Kings",
    "2 Kings",
    "1 Chronicles",
    "2 Chronicles",
    "Ezra",
    "Nehemiah",
    "Esther",
    
    // Poetic Books
    "Job",
    "Psalms",
    "Proverbs",
    "Ecclesiastes",
    "Song of Solomon",
    
    // Major Prophets
    "Isaiah",
    "Jeremiah",
    "Lamentations",
    "Ezekiel",
    "Daniel",
    
    // Minor Prophets
    "Hosea",
    "Joel",
    "Amos",
    "Obadiah",
    "Jonah",
    "Micah",
    "Nahum",
    "Habakkuk",
    "Zephaniah",
    "Haggai",
    "Zechariah",
    "Malachi",
    
    // New Testament
    
    // Gospels
    "Matthew",
    "Mark",
    "Luke",
    "John",
    
    // Acts
    "Acts of the Apostles",
    
    // Pauline Epistles
    "Romans",
    "1 Corinthians",
    "2 Corinthians",
    "Galatians",
    "Ephesians",
    "Philippians",
    "Colossians",
    "1 Thessalonians",
    "2 Thessalonians",
    "1 Timothy",
    "2 Timothy",
    "Titus",
    "Philemon",
    
    // General Epistles
    "Hebrews",
    "James",
    "1 Peter",
    "2 Peter",
    "1 John",
    "2 John",
    "3 John",
    "Jude",
    
    // Revelation
    "Revelation",
    ];

    let reformed_systematic_theology_topics: [&str; 73] = [
    "Scripture and Revelation",
    "The Nature and Attributes of God",
    "The Trinity",
    "Decree of God",
    "Creation",
    "Providence",
    "The Fall of Man",
    "Sin",
    "Covenant Theology",
    "Christology",
    "The Person of Christ",
    "The Work of Christ",
    "The Offices of Christ",
    "The Atonement",
    "Election and Predestination",
    "The Gospel Call and Effective Calling",
    "Regeneration",
    "Conversion (Faith and Repentance)",
    "Justification by Faith Alone",
    "Adoption",
    "Sanctification",
    "Perseverance of the Saints",
    "Assurance of Salvation",
    "The Law and the Gospel",
    "Christian Liberty and Liberty of Conscience",
    "The Church",
    "The Marks of the Church",
    "Church Government",
    "The Means of Grace",
    "Baptism",
    "The Lord's Supper",
    "Worship and the Sabbath",
    "The Lord's Day",
    "Prayer",
    "The Kingdom of God",
    "The Last Things (Eschatology)",
    "The Resurrection",
    "The Final Judgment",
    "Heaven",
    "Hell",
    "The New Creation",
    "Theology Proper",
    "Anthropology (Doctrine of Man)",
    "Hamartiology (Doctrine of Sin)",
    "Soteriology (Doctrine of Salvation)",
    "The Order of Salvation",
    "Union with Christ",
    "The Imputation of Christ's Righteousness",
    "Limited Atonement",
    "Irresistible Grace",
    "The Communion of Saints",
    "The Sacraments",
    "Ecclesiology (Doctrine of the Church)",
    "The Mission of the Church",
    "The Visibility and Invisibility of the Church",
    "Church Discipline",
    "The Ministry and the Ministers",
    "Spiritual Gifts",
    "Eschatology (Doctrine of Last Things)",
    "The Second Coming of Christ",
    "The Millennium",
    "The Rapture",
    "The State, Civil Government and Society",
    "Christian Ethics",
    "Marriage and Family",
    "Work and Vocation",
    "The Stewardship of Resources",
    "Theological Method",
    "History of Reformation Theology",
    "Contemporary Issues in Theology",
    "Apologetics and Evangelism",
    "The Solas of the Reformation",
    "The Doctrines of Grace",
    ];

    let random_topic = get_random_topic(&reformed_systematic_theology_topics);
    println!("Random topic: {}", random_topic);
}
