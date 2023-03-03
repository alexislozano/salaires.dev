pub enum I18n {
    Company,
    Location,
    Compensation,
    Level,
    Title,
    CompanyXp,
    TotalXp,
    Date,
    IAddMySalary,
    ShouldNotBeEmpty,
    ShouldBeANumber,
    ShouldBePositive,
    Optional,
    Send,
    Sending,
    ThisPageDoesNotExist,
    InsertIsDownForNow,
    SalaryInserted,
    SalaryInstertingError,
    LoadingData,
    InYears,
    InEuros,
    CompensationHelp,
    TitlePlaceholder,
    TheSiteIsInMaintenance,
    EmailShouldContainAnAt,
    Email,
    EmailShouldBePro,
    TokenConfirmationSuccess,
    TokenConfirmationError,
    EmailExplanation,
    Junior,
    Mid,
    Senior,
    LevelIsNotInTheProvidedChoices,
}

impl I18n {
    pub fn translate(&self) -> &'static str {
        match self {
            Self::Company => "Entreprise",
            Self::Location => "Localisation",
            Self::Compensation => "Rémunération brute",
            Self::Level => "Niveau",
            Self::Title => "Intitulé du poste",
            Self::CompanyXp => "Expérience entreprise",
            Self::TotalXp => "Expérience totale",
            Self::Date => "Date d'ajout",
            Self::IAddMySalary => "J'ajoute mon salaire",
            Self::ShouldNotBeEmpty => "Ce champ ne peut pas être vide",
            Self::ShouldBeANumber => "Ce champ doit être un nombre",
            Self::ShouldBePositive => "Ce nombre doit être positif",
            Self::Optional => "Optionnel",
            Self::Send => "Envoyer",
            Self::Sending => "En cours d'envoi...",
            Self::ThisPageDoesNotExist => "Cette page n'existe pas",
            Self::InsertIsDownForNow => "Il n'est plus possible d'ajouter des salaires pour le moment.",
            Self::SalaryInserted => "Le salaire a été ajouté, un email de confirmation a été envoyé",
            Self::SalaryInstertingError => "Le salaire n'a pas pu être ajouté",
            Self::LoadingData => "Chargement des données...",
            Self::InYears => "en années",
            Self::InEuros => "en €",
            Self::CompensationHelp => "fixe + variable en € / an",
            Self::TheSiteIsInMaintenance => "Le site est en maintenance",
            Self::TitlePlaceholder => "Dev fullstack",
            Self::EmailShouldContainAnAt => "Une adresse email doit comporter un @",
            Self::Email => "Email",
            Self::EmailShouldBePro => "L'adresse email doit être professionnelle",
            Self::TokenConfirmationSuccess => "Le salaire a bien été confirmé, il sera publié prochainement",
            Self::TokenConfirmationError => "Le salaire n'a pas pu être confirmé",
            Self::EmailExplanation => "L'adresse email et l'entreprise renseignées doivent correspondre. Sans cela, le salaire ne sera pas publié.",
            Self::Junior => "Junior",
            Self::Mid => "Intermédiaire",
            Self::Senior => "Senior",
            Self::LevelIsNotInTheProvidedChoices => "Le niveau n'est pas dans les choix proposés."
        }
    }
}
