FROM gitpod/workspace-full

RUN sudo apt update -q && \
    sudo apt upgrade -yq && \
    sudo apt install python -yq &&\
    curl https://raw.githubusercontent.com/rapid7/metasploit-omnibus/master/config/templates/metasploit-framework-wrappers/msfupdate.erb > msfinstall && \
    sudo chmod 755 msfinstall && \
    sudo ./msfinstall && \
    printf "\n" | msfdb init

EXPOSE 5553
CMD printf "\nno\n" | msfrpcd -f -U user -P password -a 0.0.0.0 -p 5553
