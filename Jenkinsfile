pipeline {
    agent any

    stages {
        stage('Hello world') {
            steps {
              sh 'echo pee pee'
            }
        }

        // stage('Push image') {
        //   steps {
        //     sh 'docker push localhost:5000/ttt'
        //   }
        // }
        //
        // stage('Package') {
        //   steps {
        //     sh 'helm install ttt ttt | true'
        //     sh 'helm upgrade ttt ttt'
        //   }
        // }
    }
}
