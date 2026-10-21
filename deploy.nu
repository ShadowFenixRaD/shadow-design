#!/usr/bin/env nu

def main [] {
    let target_dir = "/var/www/shadow-design"
    let dist_dir = ($env.PWD | path join "target/dx/showcase/release/web/public")

    print $"(ansi green_bold)[1/4](ansi reset) Compilazione Dioxus Web in release..."
    dx build --platform web --release --package showcase

    if not ($dist_dir | path exists) {
        print $"(ansi red_bold)Errore:(ansi reset) Cartella 'dist/' non trovata in ($env.PWD)."
        return
    }

    print $"(ansi green_bold)[2/4](ansi reset) Preparazione directory di destinazione ($target_dir)..."
    sudo mkdir -p $target_dir

    print $"(ansi green_bold)[3/4](ansi reset) Svuotamento e copia dei nuovi asset..."
    # Pulizia preventiva dei vecchi file per evitare artefatti obsoleti
    sudo rm -rf $target_dir
    sudo mkdir -p $target_dir
    # Copia ricorsiva del contenuto di dist/ nella cartella web root
    sudo cp -r $dist_dir $target_dir

    print $"(ansi green_bold)[4/4](ansi reset) Aggiornamento permessi cartella..."
    # Assegna la proprietà all'utente web server (tipicamente www-data o nginx)
    sudo chown -R nginx:nginx $target_dir
    sudo chmod -R 755 $target_dir

    print $"(ansi green_bold)Deploy completato con successo!(ansi reset) Sito aggiornato in ($target_dir)"
}
