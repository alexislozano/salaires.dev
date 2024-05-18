import { UnreachableCaseError } from "@utils";

export type I18n =
    | "company"
    | "location"
    | "compensation"
    | "level"
    | "title"
    | "company_xp"
    | "total_xp"
    | "date"
    | "i_add_my_salary"
    | "should_not_be_empty"
    | "should_be_a_number"
    | "should_be_positive"
    | "should_be_an_integer"
    | "optional"
    | "send"
    | "this_page_does_not_exist"
    | "insert_is_down_for_now"
    | "salary_inserted"
    | "salary_inserting_error"
    | "in_years"
    | "compensation_help"
    | "title_placeholder"
    | "the_site_is_in_maintenance"
    | "email_should_contain_an_at"
    | "email"
    | "email_should_be_pro"
    | "token_confirmation_success"
    | "token_confirmation_error"
    | "email_explanation"
    | "junior"
    | "mid"
    | "senior"
    | "level_is_not_in_the_provided_choices"
    | "salaries_fetching_error"
    | "form_fetching_error"
    | "validation_error"
    | "sort_error"
    | "remote_terms"
    | "no_remote"
    | "full_remote"
    | "partial_remote"
    | "undefined"
    | "week"
    | "month"
    | "remote_day_count"
    | "remote_base"
    | "remote_location"
    | "days_per"
    | "remote_variant_is_not_in_the_provided_choices"
    | "remote_day_count_should_be_positive"
    | "remote_day_count_should_be_a_number"
    | "remote_day_count_should_be_an_integer"
    | "remote_base_should_be_a_week_or_a_month"
    | "remote_day_count_should_be_between_1_and_7"
    | "remote_day_count_should_be_between_1_and_31"
    | "remote_location_should_be_remote_or_office"
    | "in"
    | "remote"
    | "office"
    ;

export const I18n = {
    translate(i18n: I18n): string {
        switch (i18n) {
            case "company": return "Entreprise";
            case "location": return "Localisation";
            case "compensation": return "Rémunération brute";
            case "level": return "Niveau";
            case "title": return "Intitulé du poste";
            case "company_xp": return "Expérience entreprise";
            case "total_xp": return "Expérience totale";
            case "date": return "Date d'ajout";
            case "i_add_my_salary": return "J'ajoute mon salaire";
            case "should_not_be_empty": return "Ce champ ne peut pas être vide";
            case "should_be_a_number": return "Ce champ doit être un nombre";
            case "should_be_positive": return "Ce nombre doit être positif";
            case "should_be_an_integer": return "Ce nombre doit être entier";
            case "optional": return "Optionnel";
            case "send": return "Envoyer";
            case "this_page_does_not_exist": return "Cette page n'existe pas";
            case "insert_is_down_for_now": return "Il n'est plus possible d'ajouter des salaires pour le moment.";
            case "salary_inserted": return "Le salaire a été enregistré, cliquez sur le lien qui a été envoyé à l'adresse email renseignée pour le confirmer";
            case "salary_inserting_error": return "Le salaire n'a pas pu être ajouté";
            case "in_years": return "en années";
            case "compensation_help": return "fixe + variable en € / an";
            case "the_site_is_in_maintenance": return "Le site est en maintenance";
            case "title_placeholder": return "Dev fullstack";
            case "email_should_contain_an_at": return "Une adresse email doit comporter un @";
            case "email": return "Email";
            case "email_should_be_pro": return "L'adresse email doit être professionnelle";
            case "token_confirmation_success": return "Le salaire a bien été confirmé, il sera publié prochainement";
            case "token_confirmation_error": return "Le salaire n'a pas pu être confirmé";
            case "email_explanation": return "L'adresse email et l'entreprise renseignées doivent correspondre. De plus, vous devez avoir accès à l'adresse email renseignée pour cliquer sur le lien de confirmation qui y sera envoyé. Sans cela, le salaire ne sera pas publié.";
            case "junior": return "Junior";
            case "mid": return "Mid";
            case "senior": return "Senior";
            case "level_is_not_in_the_provided_choices": return "Le niveau n'est pas dans les choix proposés.";
            case "salaries_fetching_error": return "Les salaires n'ont pas pu être chargés";
            case "form_fetching_error": return "Le formulaire n'a pas pu être chargé";
            case "validation_error": return "Une erreur inconnue est survenue lors de la validation du formulaire";
            case "sort_error": return "Une erreur inconnue est survenue lors du tri des salaires";
            case "remote_terms": return "Modalités de télétravail";
            case "no_remote": return "Pas de télétravail";
            case "full_remote": return "Télétravail complet";
            case "partial_remote": return "Télétravail partiel";
            case "undefined": return "Non renseigné";
            case "week": return "semaine";
            case "month": return "mois";
            case "remote_day_count": return "Nombre de jours";
            case "remote_base": return "Base de temps de décompte des jours";
            case "remote_location": return "Lieu de décompte des jours"
            case "days_per": return "jour(s) par";
            case "remote_variant_is_not_in_the_provided_choices": return "La variante de télétravail n'est pas dans les choix proposés";
            case "remote_day_count_should_be_positive": return "Le nombre de jours doit être positif";
            case "remote_day_count_should_be_a_number": return "Le nombre de jours doit être un nombre";
            case "remote_day_count_should_be_an_integer": return "Le nombre de jours doit être entier";
            case "remote_base_should_be_a_week_or_a_month": return "Le base de décompte des jours doit être la semaine ou le mois";
            case "remote_day_count_should_be_between_1_and_7": return "Le nombre de jours doit être compris entre 1 et 7";
            case "remote_day_count_should_be_between_1_and_31": return "Le nombre de jours doit être compris entre 1 et 31";
            case "remote_location_should_be_remote_or_office": return "Le lieu de décompte des jours doit être présentiel ou télétravail";
            case "in": return "en";
            case "remote": return "télétravail";
            case "office": return "présentiel";
            default: throw new UnreachableCaseError(i18n);
        }
    }
}
