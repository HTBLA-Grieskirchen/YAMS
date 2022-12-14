-- ------------------------------
-- OPTION
-- ------------------------------

OPTION IMPORT;

-- ------------------------------
-- TABLE: address
-- ------------------------------

DEFINE TABLE address SCHEMAFULL;

DEFINE FIELD city ON address TYPE record(city) ASSERT $after != NULL;
DEFINE FIELD extra ON address TYPE string ASSERT $after != NULL;
DEFINE FIELD street ON address TYPE string ASSERT $after != NULL;

-- ------------------------------
-- TABLE: animal
-- ------------------------------

DEFINE TABLE animal SCHEMAFULL;

DEFINE FIELD birthdate ON animal TYPE datetime;
DEFINE FIELD name ON animal TYPE string ASSERT $after != NULL;
DEFINE FIELD race ON animal TYPE record(race) ASSERT $after != NULL;

-- ------------------------------
-- TABLE: animal_type
-- ------------------------------

DEFINE TABLE animal_type SCHEMAFULL;

DEFINE FIELD description ON animal_type TYPE string ASSERT $after != NULL;

-- ------------------------------
-- TABLE: city
-- ------------------------------

DEFINE TABLE city SCHEMAFULL;

DEFINE FIELD country ON city TYPE record(country) ASSERT $after != NULL;
DEFINE FIELD name ON city TYPE string ASSERT $after != NULL;
DEFINE FIELD plz ON city TYPE string ASSERT $after != NULL;

-- ------------------------------
-- TABLE: client
-- ------------------------------



DEFINE
TABLE client SCHEMAFULL;

DEFINE
FIELD birthdate ON client TYPE datetime ASSERT $after != NULL;
DEFINE
FIELD consent ON client TYPE record(pdf) ASSERT $after != NULL;
DEFINE
FIELD email ON client TYPE string ASSERT $after != NULL AND is::email($after);
DEFINE
FIELD first_name ON client TYPE string ASSERT $after != NULL;
DEFINE
FIELD last_name ON client TYPE string ASSERT $after != NULL;
DEFINE
FIELD mobile_number ON client TYPE string ASSERT $after != NULL;
DEFINE
FIELD address ON client TYPE record(address) ASSERT $after != NULL;
DEFINE 
FIELD animals ON client VALUE [];

-- ------------------------------
-- TABLE: client_file
-- ------------------------------

DEFINE
TABLE client_file SCHEMAFULL;

DEFINE
FIELD first_consultation ON client_file TYPE datetime ASSERT $after != NULL;
DEFINE
FIELD extra ON client_file TYPE string;
DEFINE
FIELD client ON client_file TYPE record(client) ASSERT $after != NULL;
DEFINE FIELD treatment ON client_file VALUE [];

-- ------------------------------
-- TABLE: country
-- ------------------------------

DEFINE TABLE country SCHEMAFULL;

DEFINE FIELD name ON country TYPE string ASSERT $after != NULL AND string::trim($after) != "";
DEFINE FIELD short ON country TYPE string ASSERT $after != NULL AND string::trim($after) != "" AND (count((SELECT id FROM country WHERE short = $after)) < 1 OR $after == $before);

-- ------------------------------
-- TABLE: event
-- ------------------------------

DEFINE TABLE event SCHEMAFULL;

DEFINE FIELD date ON event TYPE datetime ASSERT $after != NULL;
DEFINE FIELD location ON event TYPE record(addresse) ASSERT $after != NULL;
DEFINE FIELD location_name ON event TYPE string;
DEFINE FIELD max_participants ON event TYPE int;
DEFINE FIELD seminar ON event TYPE record(seminar) ASSERT $after != NULL;

-- ------------------------------
-- TABLE: image
-- ------------------------------

DEFINE TABLE image SCHEMAFULL;

DEFINE FIELD data ON image TYPE string ASSERT $after != NULL;
DEFINE FIELD height ON image TYPE int ASSERT $after != NULL;
DEFINE FIELD width ON image TYPE int ASSERT $after != NULL;

-- ------------------------------
-- TABLE: pdf
-- ------------------------------

DEFINE TABLE pdf SCHEMAFULL;

DEFINE FIELD data ON pdf TYPE string ASSERT $after != NULL;

-- ------------------------------
-- TABLE: invoice
-- ------------------------------

DEFINE TABLE invoice SCHEMAFULL;

DEFINE FIELD sum ON invoice VALUE <future> { math::sum(<-paid_in.cost) };
DEFINE FIELD is_settled ON invoice TYPE bool ASSERT $after != null;
DEFINE FIELD issue_date ON invoice TYPE datetime ASSERT $after != NULL;
DEFINE FIELD number ON invoice TYPE int ASSERT $after != NULL;
DEFINE FIELD receipt ON invoice TYPE record(pdf);
DEFINE FIELD transaction_type ON invoice TYPE record(transaction_type) ASSERT $after != NULL;

-- ------------------------------
-- TABLE: paid
-- ------------------------------

DEFINE TABLE paid_in SCHEMAFULL;

DEFINE FIELD out ON paid TYPE record(invoice) ASSERT $after != NULL;
DEFINE FIELD cost ON paid TYPE decimal ASSERT $after != NULL;
DEFINE FIELD payer ON paid TYPE record(client) ASSERT $after != NULL;

-- ------------------------------
-- TABLE: participated
-- ------------------------------

DEFINE TABLE participated_in SCHEMAFULL;

DEFINE FIELD in ON participated_in TYPE record(client) ASSERT $after != NULL;
DEFINE FIELD out ON participated_in TYPE record(event) ASSERT $after != NULL;

-- ------------------------------
-- TABLE: product
-- ------------------------------

DEFINE TABLE product SCHEMAFULL;

DEFINE FIELD description ON product TYPE string ASSERT $after != NULL;
DEFINE FIELD price ON product TYPE decimal ASSERT $after != NULL;
DEFINE FIELD type ON product TYPE record(product_type) ASSERT $after != NULL;

-- ------------------------------
-- TABLE: product_type
-- ------------------------------

DEFINE TABLE product_type SCHEMAFULL;

DEFINE FIELD description ON product_type TYPE string ASSERT $after != NULL;

-- ------------------------------
-- TABLE: purchased
-- ------------------------------

DEFINE TABLE purchased SCHEMAFULL;

DEFINE FIELD in ON purchased TYPE record(client) ASSERT $after != NULL;
DEFINE FIELD out ON purchased TYPE record(product) ASSERT $after != NULL;
DEFINE FIELD amount ON purchased TYPE int ASSERT $after != NULL;

-- ------------------------------
-- TABLE: race
-- ------------------------------

DEFINE TABLE race SCHEMAFULL;

DEFINE FIELD description ON race TYPE string ASSERT $after != NULL;
DEFINE FIELD type ON race TYPE record(animal_type) ASSERT $after != NULL;

-- ------------------------------
-- TABLE: seminar
-- ------------------------------

DEFINE TABLE seminar SCHEMAFULL;

DEFINE FIELD duration ON seminar TYPE duration;
DEFINE FIELD price ON seminar TYPE decimal ASSERT $after != NULL;
DEFINE FIELD title ON seminar TYPE string ASSERT $after != NULL;

-- ------------------------------
-- TABLE: transaction_type
-- ------------------------------

DEFINE TABLE transaction_type SCHEMAFULL;

DEFINE FIELD description ON transaction_type TYPE string ASSERT $after != NULL;
DEFINE FIELD outgoing ON transaction_type TYPE bool ASSERT $after != NULL;
DEFINE FIELD short ON transaction_type TYPE string ASSERT $after != NULL;

-- ------------------------------
-- TABLE: treatment
-- ------------------------------

DEFINE TABLE treatment SCHEMAFULL;

DEFINE FIELD description ON treatment TYPE string ASSERT $after != NULL;
DEFINE FIELD duration ON treatment TYPE duration;
DEFINE FIELD price ON treatment TYPE number ASSERT $after != NULL;
DEFINE FIELD symptomatic ON treatment TYPE string;
DEFINE FIELD targets_animal ON treatment TYPE bool ASSERT $after != NULL;
DEFINE FIELD type ON treatment TYPE record(treatment_type) ASSERT $after != NULL;

-- ------------------------------
-- TABLE: treatment_appointment
-- ------------------------------

DEFINE TABLE treatment_appointment SCHEMAFULL;

DEFINE FIELD cost ON treatment_appointment TYPE decimal ASSERT $after != NULL;
DEFINE FIELD date ON treatment_appointment TYPE datetime ASSERT $after != NULL;
DEFINE FIELD extra ON treatment_appointment TYPE string;
DEFINE FIELD treatment ON treatment_appointment TYPE record(treatment) ASSERT $after != NULL;

-- ------------------------------
-- TABLE: treatment_type
-- ------------------------------

DEFINE TABLE treatment_type SCHEMAFULL;

DEFINE FIELD description ON treatment_type TYPE string ASSERT $after != NULL;

-- ------------------------------
-- TRANSACTION
-- ------------------------------

BEGIN TRANSACTION;

-- ------------------------------
-- TABLE DATA: addresse
-- ------------------------------


-- ------------------------------
-- TABLE DATA: animal
-- ------------------------------


-- ------------------------------
-- TABLE DATA: animal_type
-- ------------------------------


-- ------------------------------
-- TABLE DATA: city
-- ------------------------------


-- ------------------------------
-- TABLE DATA: client
-- ------------------------------


-- ------------------------------
-- TABLE DATA: client_file
-- ------------------------------


-- ------------------------------
-- TABLE DATA: event
-- ------------------------------


-- ------------------------------
-- TABLE DATA: image
-- ------------------------------


-- ------------------------------
-- TABLE DATA: invoice
-- ------------------------------


-- ------------------------------
-- TABLE DATA: country
-- ------------------------------


-- ------------------------------
-- TABLE DATA: paid
-- ------------------------------


-- ------------------------------
-- TABLE DATA: participated
-- ------------------------------


-- ------------------------------
-- TABLE DATA: product
-- ------------------------------


-- ------------------------------
-- TABLE DATA: product_type
-- ------------------------------


-- ------------------------------
-- TABLE DATA: purchased
-- ------------------------------


-- ------------------------------
-- TABLE DATA: race
-- ------------------------------


-- ------------------------------
-- TABLE DATA: seminar
-- ------------------------------


-- ------------------------------
-- TABLE DATA: transaction_type
-- ------------------------------


-- ------------------------------
-- TABLE DATA: treatment
-- ------------------------------


-- ------------------------------
-- TABLE DATA: treatment_appointment
-- ------------------------------


-- ------------------------------
-- TABLE DATA: treatment_type
-- ------------------------------


-- ------------------------------
-- TRANSACTION
-- ------------------------------

COMMIT TRANSACTION;

