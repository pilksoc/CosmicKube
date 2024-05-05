pipeline {
    agent {
        docker {
            image 'barichello/godot-ci:4.2.1'

    }        }
    environment {
        GODOT_VERSION = '4.2.1'
        EXPORT_NAME = 'CosmicKube'
        PROJECT_PATH = 'game-source'
    }

    stages {
        stage('Setup') {
            steps {
                sh '''mkdir -v -p ~/.local/share/godot/export_templates/
                mv /root/.local/share/godot/export_templates/${GODOT_VERSION}.stable ~/.local/share/godot/export_templates/${GODOT_VERSION}.stable
                '''
            }
        }
        stage('Web Build') {
            steps {
                sh '''cd game-source-redeux || exit 1
                pnpm i
                pnpm build
                cp -r dist/* /home/static/kube-frontend/
                '''
            }
        }
    }
}
