
ARG ES_VERSION=7.8.0
ARG SUDACHI_PLUGIN_VER=2.0.2

FROM  docker.elastic.co/elasticsearch/elasticsearch:${ES_VERSION}

ARG ES_VERSION
ARG SUDACHI_PLUGIN_VER

COPY sudachi.json /usr/share/elasticsearch/config/sudachi/

RUN  bin/elasticsearch-plugin install https://github.com/WorksApplications/elasticsearch-sudachi/releases/download/v${ES_VERSION}-${SUDACHI_PLUGIN_VER}/analysis-sudachi-${ES_VERSION}-${SUDACHI_PLUGIN_VER}.zip && \
    curl -OL https://object-storage.tyo2.conoha.io/v1/nc_2520839e1f9641b08211a5c85243124a/sudachi/sudachi-dictionary-20200330-core.zip && \
    curl -OL https://object-storage.tyo2.conoha.io/v1/nc_2520839e1f9641b08211a5c85243124a/sudachi/sudachi-dictionary-20200330-full.zip &&  \
    unzip -o -d config/sudachi -j  'sudachi-dictionary*.zip'  && \
    rm -f sudachi-dictionary*.zip