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
            case "salary_inserted": return "Le salaire a été ajouté, un email de confirmation a été envoyé";
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
            case "email_explanation": return "L'adresse email et l'entreprise renseignées doivent correspondre. Sans cela, le salaire ne sera pas publié.";
            case "junior": return "Junior";
            case "mid": return "Mid";
            case "senior": return "Senior";
            case "level_is_not_in_the_provided_choices": return "Le niveau n'est pas dans les choix proposés.";
            case "salaries_fetching_error": return "Les salaires n'ont pas pu être chargés";
            case "form_fetching_error": return "Le formulaire n'a pas pu être chargé";
            case "validation_error": return "Une erreur inconnue est survenue lors de la validation du formulaire";
            case "sort_error": return "Une erreur inconnue est survenue lors du tri des salaires";
            default: throw new UnreachableCaseError(i18n);
        }
    }
}
