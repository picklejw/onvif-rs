<?xml version="1.0" encoding="utf-8"?>
<?xml-stylesheet type="text/xsl" href="../ver20/util/onvif-wsdl-viewer.xsl"?>
<!--
Copyright (c) 2008-2014 by ONVIF: Open Network Video Interface Forum. All rights reserved.

Recipients of this document may copy, distribute, publish, or display this document so long as this copyright notice, license and disclaimer are retained with all copies of the document. No license is granted to modify this document.

THIS DOCUMENT IS PROVIDED "AS IS," AND THE CORPORATION AND ITS MEMBERS AND THEIR AFFILIATES, MAKE NO REPRESENTATIONS OR WARRANTIES, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO, WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE, NON-INFRINGEMENT, OR TITLE; THAT THE CONTENTS OF THIS DOCUMENT ARE SUITABLE FOR ANY PURPOSE; OR THAT THE IMPLEMENTATION OF SUCH CONTENTS WILL NOT INFRINGE ANY PATENTS, COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS.
IN NO EVENT WILL THE CORPORATION OR ITS MEMBERS OR THEIR AFFILIATES BE LIABLE FOR ANY DIRECT, INDIRECT, SPECIAL, INCIDENTAL, PUNITIVE OR CONSEQUENTIAL DAMAGES, ARISING OUT OF OR RELATING TO ANY USE OR DISTRIBUTION OF THIS DOCUMENT, WHETHER OR NOT (1) THE CORPORATION, MEMBERS OR THEIR AFFILIATES HAVE BEEN ADVISED OF THE POSSIBILITY OF SUCH DAMAGES, OR (2) SUCH DAMAGES WERE REASONABLY FORESEEABLE, AND ARISING OUT OF OR RELATING TO ANY USE OR DISTRIBUTION OF THIS DOCUMENT.  THE FOREGOING DISCLAIMER AND LIMITATION ON LIABILITY DO NOT APPLY TO, INVALIDATE, OR LIMIT REPRESENTATIONS AND WARRANTIES MADE BY THE MEMBERS AND THEIR RESPECTIVE AFFILIATES TO THE CORPORATION AND OTHER MEMBERS IN CERTAIN WRITTEN POLICIES OF THE CORPORATION.
-->
<wsdl:definitions xmlns:wsdl="http://schemas.xmlsoap.org/wsdl/" xmlns:soap="http://schemas.xmlsoap.org/wsdl/soap12/" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:tse="http://www.onvif.org/ver10/search/wsdl" targetNamespace="http://www.onvif.org/ver10/search/wsdl">
	<wsdl:types>
		<xs:schema targetNamespace="http://www.onvif.org/ver10/search/wsdl" xmlns:tt="http://www.onvif.org/ver10/schema" xmlns:xs="http://www.w3.org/2001/XMLSchema" elementFormDefault="qualified" version="2.4.2">
			<xs:import namespace="http://www.onvif.org/ver10/schema" schemaLocation="../ver10/schema/onvif.xsd"/>
			<!--  Message Request/Responses elements  -->
			<!--===============================-->
			<xs:element name="GetServiceCapabilities">
				<xs:complexType>
					<xs:sequence/>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetServiceCapabilitiesResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Capabilities" type="tse:Capabilities">
							<xs:annotation>
								<xs:documentation>The capabilities for the search service is returned in the Capabilities element.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:complexType name="Capabilities">
				<xs:sequence>
					<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>
				</xs:sequence>
				<xs:attribute name="MetadataSearch" type="xs:boolean"/>
				<xs:attribute name="GeneralStartEvents" type="xs:boolean"><xs:annotation><xs:documentation> Indicates support for general virtual property events in the FindEvents method.</xs:documentation></xs:annotation></xs:attribute>
				<xs:anyAttribute processContents="lax"/>
			</xs:complexType>
			<xs:element name="Capabilities" type="tse:Capabilities"/>
			<!--===============================-->
			<xs:element name="GetRecordingSummary">
				<xs:complexType>
					<xs:sequence>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetRecordingSummaryResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Summary" type="tt:RecordingSummary"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetRecordingInformation">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="RecordingToken" type="tt:RecordingReference"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetRecordingInformationResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="RecordingInformation" type="tt:RecordingInformation"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetMediaAttributes">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="RecordingTokens" type="tt:RecordingReference" minOccurs="0" maxOccurs="unbounded"/>
						<xs:element name="Time" type="xs:dateTime"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetMediaAttributesResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="MediaAttributes" type="tt:MediaAttributes" minOccurs="0" maxOccurs="unbounded"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="FindRecordings">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Scope" type="tt:SearchScope">
							<xs:annotation>
								<xs:documentation>Scope defines the dataset to consider for this search.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="MaxMatches" type="xs:int" minOccurs="0">
							<xs:annotation>
								<xs:documentation>The search will be completed after this many matches. If not specified, the search will continue until reaching the endpoint or until the session expires.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="KeepAliveTime" type="xs:duration">
							<xs:annotation>
								<xs:documentation>The time the search session will be kept alive after responding to this and subsequent requests. A device shall support at least values up to ten seconds.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="FindRecordingsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="SearchToken" type="tt:JobToken"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetRecordingSearchResults">
				<xs:annotation>
					<xs:documentation>Gets results from a particular recording listingession.</xs:documentation>
				</xs:annotation>
				<xs:complexType>
					<xs:sequence>
						<xs:element name="SearchToken" type="tt:JobToken">
							<xs:annotation>
								<xs:documentation>The search session to get results from.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="MinResults" type="xs:int" minOccurs="0">
							<xs:annotation>
								<xs:documentation>The minimum number of results to return in one response.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="MaxResults" type="xs:int" minOccurs="0">
							<xs:annotation>
								<xs:documentation>The maximum number of results to return in one response.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="WaitTime" type="xs:duration" minOccurs="0">
							<xs:annotation>
								<xs:documentation>The maximum time before responding to the request, even if the MinResults parameter is not fulfilled.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetRecordingSearchResultsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ResultList" type="tt:FindRecordingResultList"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="FindEvents">
				<xs:annotation>
					<xs:documentation>Starts a search session and specifies the search parameters.</xs:documentation>
				</xs:annotation>
				<xs:complexType>
					<xs:sequence>
						<xs:element name="StartPoint" type="xs:dateTime">
							<xs:annotation>
								<xs:documentation>The point of time where the search will start.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="EndPoint" type="xs:dateTime" minOccurs="0">
							<xs:annotation>
								<xs:documentation>The point of time where the search will stop. This can be a time before the StartPoint, in which case the search is performed backwards in time.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="Scope" type="tt:SearchScope"/>
						<xs:element name="SearchFilter" type="tt:EventFilter"/>
						<xs:element name="IncludeStartState" type="xs:boolean">
							<xs:annotation>
								<xs:documentation>Setting IncludeStartState to true means that the server should return virtual events representing the start state for any recording included in the scope. Start state events are limited to the topics defined in the SearchFilter that have the IsProperty flag set to true.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="MaxMatches" type="xs:int" minOccurs="0">
							<xs:annotation>
								<xs:documentation>The search will be completed after this many matches. If not specified, the search will continue until reaching the endpoint or until the session expires.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="KeepAliveTime" type="xs:duration">
							<xs:annotation>
								<xs:documentation>The time the search session will be kept alive after responding to this and subsequent requests. A device shall support at least values up to ten seconds.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="FindEventsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="SearchToken" type="tt:JobToken">
							<xs:annotation>
								<xs:documentation>A unique reference to the search session created by this request.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetEventSearchResults">
				<xs:annotation>
					<xs:documentation>Gets results from a particular search session.</xs:documentation>
				</xs:annotation>
				<xs:complexType>
					<xs:sequence>
						<xs:element name="SearchToken" type="tt:JobToken">
							<xs:annotation>
								<xs:documentation>The search session to get results from.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="MinResults" type="xs:int" minOccurs="0">
							<xs:annotation>
								<xs:documentation>The minimum number of results to return in one response.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="MaxResults" type="xs:int" minOccurs="0">
							<xs:annotation>
								<xs:documentation>The maximum number of results to return in one response.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="WaitTime" type="xs:duration" minOccurs="0">
							<xs:annotation>
								<xs:documentation>The maximum time before responding to the request, even if the MinResults parameter is not fulfilled.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetEventSearchResultsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ResultList" type="tt:FindEventResultList"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="FindPTZPosition">
				<xs:annotation>
					<xs:documentation>Starts a search session and specifies the search parameters.</xs:documentation>
				</xs:annotation>
				<xs:complexType>
					<xs:sequence>
						<xs:element name="StartPoint" type="xs:dateTime">
							<xs:annotation>
								<xs:documentation>The point of time where the search will start.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="EndPoint" type="xs:dateTime" minOccurs="0">
							<xs:annotation>
								<xs:documentation>The point of time where the search will stop. This can be a time before the StartPoint, in which case the search is performed backwards in time.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="Scope" type="tt:SearchScope"/>
						<xs:element name="SearchFilter" type="tt:PTZPositionFilter"/>
						<xs:element name="MaxMatches" type="xs:int" minOccurs="0">
							<xs:annotation>
								<xs:documentation>The search will be completed after this many matches. If not specified, the search will continue until reaching the endpoint or until the session expires.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="KeepAliveTime" type="xs:duration">
							<xs:annotation>
								<xs:documentation>The time the search session will be kept alive after responding to this and subsequent requests. A device shall support at least values up to ten seconds.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="FindPTZPositionResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="SearchToken" type="tt:JobToken">
							<xs:annotation>
								<xs:documentation>A unique reference to the search session created by this request.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetPTZPositionSearchResults">
				<xs:annotation>
					<xs:documentation>Gets results from a particular search session.</xs:documentation>
				</xs:annotation>
				<xs:complexType>
					<xs:sequence>
						<xs:element name="SearchToken" type="tt:JobToken">
							<xs:annotation>
								<xs:documentation>The search session to get results from.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="MinResults" type="xs:int" minOccurs="0">
							<xs:annotation>
								<xs:documentation>The minimum number of results to return in one response.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="MaxResults" type="xs:int" minOccurs="0">
							<xs:annotation>
								<xs:documentation>The maximum number of results to return in one response.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="WaitTime" type="xs:duration" minOccurs="0">
							<xs:annotation>
								<xs:documentation>The maximum time before responding to the request, even if the MinResults parameter is not fulfilled.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetPTZPositionSearchResultsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ResultList" type="tt:FindPTZPositionResultList"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="FindMetadata">
				<xs:annotation>
					<xs:documentation>Starts a search session and specifies the search parameters.</xs:documentation>
				</xs:annotation>
				<xs:complexType>
					<xs:sequence>
						<xs:element name="StartPoint" type="xs:dateTime">
							<xs:annotation>
								<xs:documentation>The point of time where the search will start.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="EndPoint" type="xs:dateTime" minOccurs="0">
							<xs:annotation>
								<xs:documentation>The point of time where the search will stop. This can be a time before the StartPoint, in which case the search is performed backwards in time.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="Scope" type="tt:SearchScope"/>
						<xs:element name="MetadataFilter" type="tt:MetadataFilter"/>
						<xs:element name="MaxMatches" type="xs:int" minOccurs="0">
							<xs:annotation>
								<xs:documentation>The search will be completed after this many matches. If not specified, the search will continue until reaching the endpoint or until the session expires.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="KeepAliveTime" type="xs:duration">
							<xs:annotation>
								<xs:documentation>The time the search session will be kept alive after responding to this and subsequent requests. A device shall support at least values up to ten seconds.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="FindMetadataResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="SearchToken" type="tt:JobToken">
							<xs:annotation>
								<xs:documentation>A unique reference to the search session created by this request.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetMetadataSearchResults">
				<xs:annotation>
					<xs:documentation>Gets results from a particular search session.</xs:documentation>
				</xs:annotation>
				<xs:complexType>
					<xs:sequence>
						<xs:element name="SearchToken" type="tt:JobToken">
							<xs:annotation>
								<xs:documentation>The search session to get results from.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="MinResults" type="xs:int" minOccurs="0">
							<xs:annotation>
								<xs:documentation>The minimum number of results to return in one response.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="MaxResults" type="xs:int" minOccurs="0">
							<xs:annotation>
								<xs:documentation>The maximum number of results to return in one response.</xs:documentation>
							</xs:annotation>
						</xs:element>
						<xs:element name="WaitTime" type="xs:duration" minOccurs="0">
							<xs:annotation>
								<xs:documentation>The maximum time before responding to the request, even if the MinResults parameter is not fulfilled.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetMetadataSearchResultsResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="ResultList" type="tt:FindMetadataResultList"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="GetSearchState">
				<xs:annotation>
					<xs:documentation>Returns the state of an ongoing search session.</xs:documentation>
				</xs:annotation>
				<xs:complexType>
					<xs:sequence>
						<xs:element name="SearchToken" type="tt:JobToken">
							<xs:annotation>
								<xs:documentation>The search session to get the state from.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="GetSearchStateResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="State" type="tt:SearchState"/>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<xs:element name="EndSearch">
				<xs:annotation>
					<xs:documentation>Ends an ongoing search session, freeing any resources.</xs:documentation>
				</xs:annotation>
				<xs:complexType>
					<xs:sequence>
						<xs:element name="SearchToken" type="tt:JobToken">
							<xs:annotation>
								<xs:documentation>The search session to end.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<xs:element name="EndSearchResponse">
				<xs:complexType>
					<xs:sequence>
						<xs:element name="Endpoint" type="xs:dateTime">
							<xs:annotation>
								<xs:documentation>The point of time the search had reached when it was ended. It is equal to the EndPoint specified in Find-operation if the search was completed.</xs:documentation>
							</xs:annotation>
						</xs:element>
					</xs:sequence>
				</xs:complexType>
			</xs:element>
			<!--===============================-->
			<!--===============================-->
		</xs:schema>
	</wsdl:types>
	<wsdl:message name="GetServiceCapabilitiesRequest">
		<wsdl:part name="parameters" element="tse:GetServiceCapabilities"/>
	</wsdl:message>
	<wsdl:message name="GetServiceCapabilitiesResponse">
		<wsdl:part name="parameters" element="tse:GetServiceCapabilitiesResponse"/>
	</wsdl:message>
	<wsdl:message name="FindEventsRequest">
		<wsdl:part name="parameters" element="tse:FindEvents"/>
	</wsdl:message>
	<wsdl:message name="FindEventsResponse">
		<wsdl:part name="parameters" element="tse:FindEventsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetEventSearchResultsRequest">
		<wsdl:part name="parameters" element="tse:GetEventSearchResults"/>
	</wsdl:message>
	<wsdl:message name="GetEventSearchResultsResponse">
		<wsdl:part name="parameters" element="tse:GetEventSearchResultsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetSearchStateRequest">
		<wsdl:part name="parameters" element="tse:GetSearchState"/>
	</wsdl:message>
	<wsdl:message name="GetSearchStateResponse">
		<wsdl:part name="parameters" element="tse:GetSearchStateResponse"/>
	</wsdl:message>
	<wsdl:message name="EndSearchRequest">
		<wsdl:part name="parameters" element="tse:EndSearch"/>
	</wsdl:message>
	<wsdl:message name="EndSearchResponse">
		<wsdl:part name="parameters" element="tse:EndSearchResponse"/>
	</wsdl:message>
	<wsdl:message name="FindPTZPositionRequest">
		<wsdl:part name="parameters" element="tse:FindPTZPosition"/>
	</wsdl:message>
	<wsdl:message name="FindPTZPositionResponse">
		<wsdl:part name="parameters" element="tse:FindPTZPositionResponse"/>
	</wsdl:message>
	<wsdl:message name="GetPTZPositionSearchResultsRequest">
		<wsdl:part name="parameters" element="tse:GetPTZPositionSearchResults"/>
	</wsdl:message>
	<wsdl:message name="GetPTZPositionSearchResultsResponse">
		<wsdl:part name="parameters" element="tse:GetPTZPositionSearchResultsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetRecordingSummaryRequest">
		<wsdl:part name="parameters" element="tse:GetRecordingSummary"/>
	</wsdl:message>
	<wsdl:message name="GetRecordingSummaryResponse">
		<wsdl:part name="parameters" element="tse:GetRecordingSummaryResponse"/>
	</wsdl:message>
	<wsdl:message name="GetRecordingInformationRequest">
		<wsdl:part name="parameters" element="tse:GetRecordingInformation"/>
	</wsdl:message>
	<wsdl:message name="GetRecordingInformationResponse">
		<wsdl:part name="parameters" element="tse:GetRecordingInformationResponse"/>
	</wsdl:message>
	<wsdl:message name="GetMediaAttributesRequest">
		<wsdl:part name="parameters" element="tse:GetMediaAttributes"/>
	</wsdl:message>
	<wsdl:message name="GetMediaAttributesResponse">
		<wsdl:part name="parameters" element="tse:GetMediaAttributesResponse"/>
	</wsdl:message>
	<wsdl:message name="FindRecordingsRequest">
		<wsdl:part name="parameters" element="tse:FindRecordings"/>
	</wsdl:message>
	<wsdl:message name="FindRecordingsResponse">
		<wsdl:part name="parameters" element="tse:FindRecordingsResponse"/>
	</wsdl:message>
	<wsdl:message name="GetRecordingSearchResultsRequest">
		<wsdl:part name="parameters" element="tse:GetRecordingSearchResults"/>
	</wsdl:message>
	<wsdl:message name="GetRecordingSearchResultsResponse">
		<wsdl:part name="parameters" element="tse:GetRecordingSearchResultsResponse"/>
	</wsdl:message>
	<wsdl:message name="FindMetadataRequest">
		<wsdl:part name="parameters" element="tse:FindMetadata"/>
	</wsdl:message>
	<wsdl:message name="FindMetadataResponse">
		<wsdl:part name="parameters" element="tse:FindMetadataResponse"/>
	</wsdl:message>
	<wsdl:message name="GetMetadataSearchResultsRequest">
		<wsdl:part name="parameters" element="tse:GetMetadataSearchResults"/>
	</wsdl:message>
	<wsdl:message name="GetMetadataSearchResultsResponse">
		<wsdl:part name="parameters" element="tse:GetMetadataSearchResultsResponse"/>
	</wsdl:message>
	<wsdl:portType name="SearchPort">
		<!--===============================-->
		<!--===============================-->
		<wsdl:operation name="GetServiceCapabilities">
			<wsdl:documentation>Returns the capabilities of the search service. The result is returned in a typed answer.</wsdl:documentation>
			<wsdl:input message="tse:GetServiceCapabilitiesRequest"/>
			<wsdl:output message="tse:GetServiceCapabilitiesResponse"/>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetRecordingSummary">
			<wsdl:documentation>GetRecordingSummary is used to get a summary description of all recorded data. This
				operation is mandatory to support for a device implementing the recording search service.</wsdl:documentation>
			<wsdl:input message="tse:GetRecordingSummaryRequest"/>
			<wsdl:output message="tse:GetRecordingSummaryResponse"/>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetRecordingInformation">
			<wsdl:documentation>Returns information about a single Recording specified by a RecordingToken. This operation
				is mandatory to support for a device implementing the recording search service.</wsdl:documentation>
			<wsdl:input message="tse:GetRecordingInformationRequest"/>
			<wsdl:output message="tse:GetRecordingInformationResponse"/>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetMediaAttributes">
			<wsdl:documentation>Returns a set of media attributes for all tracks of the specified recordings at a specified point
				in time. Clients using this operation shall be able to use it as a non blocking operation. A
				device shall set the starttime and endtime of the MediaAttributes structure to equal values if
				calculating this range would causes this operation to block. See MediaAttributes structure for
				more information. This operation is mandatory to support for a device implementing the
				recording search service.</wsdl:documentation>
			<wsdl:input message="tse:GetMediaAttributesRequest"/>
			<wsdl:output message="tse:GetMediaAttributesResponse"/>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="FindRecordings">
			<wsdl:documentation>FindRecordings starts a search session, looking for recordings that matches the scope (See
				20.2.4) defined in the request. Results from the search are acquired using the
				GetRecordingSearchResults request, specifying the search token returned from this request.
				The device shall continue searching until one of the following occurs:<ul>
					<li>The entire time range from StartPoint to EndPoint has been searched through.</li>
					<li>The total number of matches has been found, defined by the MaxMatches parameter.</li>
					<li>The session has been ended by a client EndSession request.</li>
					<li>The session has been ended because KeepAliveTime since the last request related to
				this session has expired.</li>
				</ul>
				The order of the results is undefined, to allow the device to return results in any order they
				are found. This operation is mandatory to support for a device implementing the recording
				search service.</wsdl:documentation>
			<wsdl:input message="tse:FindRecordingsRequest"/>
			<wsdl:output message="tse:FindRecordingsResponse"/>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetRecordingSearchResults">
			<wsdl:documentation>GetRecordingSearchResults acquires the results from a recording search session previously
				initiated by a FindRecordings operation. The response shall not include results already
				returned in previous requests for the same session. If MaxResults is specified, the response
				shall not contain more than MaxResults results. The number of results relates to the number of recordings. 
				For viewing individual recorded data for a signal track use the FindEvents method.<br/>
				GetRecordingSearchResults shall block until:<ul>
					<li>
				MaxResults results are available for the response if MaxResults is specified.</li>
					<li>MinResults results are available for the response if MinResults is specified.</li>
					<li>WaitTime has expired.</li>
					<li>Search is completed or stopped.</li>
				</ul>
				This operation is mandatory to support for a device implementing the recording search service.</wsdl:documentation>
			<wsdl:input message="tse:GetRecordingSearchResultsRequest"/>
			<wsdl:output message="tse:GetRecordingSearchResultsResponse"/>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="FindEvents">
			<wsdl:documentation>FindEvents starts a search session, looking for recording events (in the scope that 
				matches the search filter defined in the request. Results from the search are
				acquired using the GetEventSearchResults request, specifying the search token returned from
				this request.<br/>
				The device shall continue searching until one of the following occurs:<ul>
					<li>
				The entire time range from StartPoint to EndPoint has been searched through.</li>
					<li>The total number of matches has been found, defined by the MaxMatches parameter.</li>
					<li>The session has been ended by a client EndSession request.</li>
					<li>The session has been ended because KeepAliveTime since the last request related to
				this session has expired.</li>
				</ul>
				Results shall be ordered by time, ascending in case of forward search, or descending in case
				of backward search. This operation is mandatory to support for a device implementing the
				recording search service.</wsdl:documentation>
			<wsdl:input message="tse:FindEventsRequest"/>
			<wsdl:output message="tse:FindEventsResponse"/>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetEventSearchResults">
			<wsdl:documentation>GetEventSearchResults acquires the results from a recording event search session previously
				initiated by a FindEvents operation. The response shall not include results already returned in
				previous requests for the same session. If MaxResults is specified, the response shall not
				contain more than MaxResults results.<br/>
				GetEventSearchResults shall block until:<ul>
					<li>
				MaxResults results are available for the response if MaxResults is specified.</li>
					<li>MinResults results are available for the response if MinResults is specified.</li>
					<li>WaitTime has expired.</li>
					<li>Search is completed or stopped.</li>
				</ul>
				This operation is mandatory to support for a device implementing the recording search service.</wsdl:documentation>
			<wsdl:input message="tse:GetEventSearchResultsRequest"/>
			<wsdl:output message="tse:GetEventSearchResultsResponse"/>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="FindPTZPosition">
			<wsdl:documentation>FindPTZPosition starts a search session, looking for ptz positions in the scope (See 20.2.4)
				that matches the search filter defined in the request. Results from the search are acquired
				using the GetPTZPositionSearchResults request, specifying the search token returned from
				this request.<br/>
				The device shall continue searching until one of the following occurs:<ul>
					<li>
				The entire time range from StartPoint to EndPoint has been searched through.</li>
					<li>The total number of matches has been found, defined by the MaxMatches parameter.</li>
					<li>The session has been ended by a client EndSession request.</li>
					<li>The session has been ended because KeepAliveTime since the last request related to
				this session has expired.</li>
				</ul>
				This operation is mandatory to support whenever CanContainPTZ is true for any metadata
				track in any recording on the device.</wsdl:documentation>
			<wsdl:input message="tse:FindPTZPositionRequest"/>
			<wsdl:output message="tse:FindPTZPositionResponse"/>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetPTZPositionSearchResults">
			<wsdl:documentation>GetPTZPositionSearchResults acquires the results from a ptz position search session
				previously initiated by a FindPTZPosition operation. The response shall not include results
				already returned in previous requests for the same session. If MaxResults is specified, the
				response shall not contain more than MaxResults results.<br/>
				GetPTZPositionSearchResults shall block until:<ul>
					<li>
				MaxResults results are available for the response if MaxResults is specified.</li>
					<li>MinResults results are available for the response if MinResults is specified.</li>
					<li>WaitTime has expired.</li>
					<li>Search is completed or stopped.</li>
				</ul>
				This operation is mandatory to support whenever CanContainPTZ is true for any metadata
				track in any recording on the device.</wsdl:documentation>
			<wsdl:input message="tse:GetPTZPositionSearchResultsRequest"/>
			<wsdl:output message="tse:GetPTZPositionSearchResultsResponse"/>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetSearchState">
			<wsdl:documentation>GetSearchState returns the current state of the specified search session. This command is deprecated .</wsdl:documentation>
			<wsdl:input message="tse:GetSearchStateRequest"/>
			<wsdl:output message="tse:GetSearchStateResponse"/>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="EndSearch">
			<wsdl:documentation>EndSearch stops and ongoing search session, causing any blocking result request to return
				and the SearchToken to become invalid. If the search was interrupted before completion, the
				point in time that the search had reached shall be returned. If the search had not yet begun,
				the StartPoint shall be returned. If the search was completed the original EndPoint supplied
				by the Find operation shall be returned. When issuing EndSearch on a FindRecordings request the 
				EndPoint is undefined and shall not be used since the FindRecordings request doesn't have 
				StartPoint/EndPoint. This operation is mandatory to support for a device implementing the 
				recording search service. 
			</wsdl:documentation>
			<wsdl:input message="tse:EndSearchRequest"/>
			<wsdl:output message="tse:EndSearchResponse"/>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="FindMetadata">
			<wsdl:documentation>FindMetadata starts a search session, looking for metadata in the scope (See 20.2.4) that
				matches the search filter defined in the request. Results from the search are acquired using
				the GetMetadataSearchResults request, specifying the search token returned from this
				request.<br/>
				The device shall continue searching until one of the following occurs:<ul>
					<li>
				The entire time range from StartPoint to EndPoint has been searched through.</li>
					<li>The total number of matches has been found, defined by the MaxMatches parameter.</li>
					<li>The session has been ended by a client EndSession request.</li>
					<li>The session has been ended because KeepAliveTime since the last request related to
				this session has expired.</li>
				</ul>
				This operation is mandatory to support if the MetaDataSearch capability is set to true in the
				SearchCapabilities structure return by the GetCapabilities command in the Device service.</wsdl:documentation>
			<wsdl:input message="tse:FindMetadataRequest"/>
			<wsdl:output message="tse:FindMetadataResponse"/>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetMetadataSearchResults">
			<wsdl:documentation>GetMetadataSearchResults acquires the results from a recording search session previously
				initiated by a FindMetadata operation. The response shall not include results already returned
				in previous requests for the same session. If MaxResults is specified, the response shall not
				contain more than MaxResults results.<br/>
				GetMetadataSearchResults shall block until:<ul>
					<li>
				MaxResults results are available for the response if MaxResults is specified.</li>
					<li>MinResults results are available for the response if MinResults is specified.</li>
					<li>WaitTime has expired.</li>
					<li>Search is completed or stopped.</li>
				</ul>
				This operation is mandatory to support if the MetaDataSearch capability is set to true in the
				SearchCapabilities structure return by the GetCapabilities command in the Device service.</wsdl:documentation>
			<wsdl:input message="tse:GetMetadataSearchResultsRequest"/>
			<wsdl:output message="tse:GetMetadataSearchResultsResponse"/>
		</wsdl:operation>
	</wsdl:portType>
	<wsdl:binding name="SearchBinding" type="tse:SearchPort">
		<soap:binding style="document" transport="http://schemas.xmlsoap.org/soap/http"/>
		<!--===============================-->
		<!--===============================-->
		<wsdl:operation name="GetServiceCapabilities">
			<soap:operation soapAction="http://www.onvif.org/ver10/search/wsdl/GetServiceCapabilities"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetRecordingSummary">
			<soap:operation soapAction="http://www.onvif.org/ver10/search/wsdl/GetRecordingSummary"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetRecordingInformation">
			<soap:operation soapAction="http://www.onvif.org/ver10/search/wsdl/GetRecordingInformation"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetMediaAttributes">
			<soap:operation soapAction="http://www.onvif.org/ver10/search/wsdl/GetMediaAttributes"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="FindRecordings">
			<soap:operation soapAction="http://www.onvif.org/ver10/search/wsdl/FindRecordings"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetRecordingSearchResults">
			<soap:operation soapAction="http://www.onvif.org/ver10/search/wsdl/GetRecordingSearchResults"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="FindEvents">
			<soap:operation soapAction="http://www.onvif.org/ver10/search/wsdl/FindEvents"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetEventSearchResults">
			<soap:operation soapAction="http://www.onvif.org/ver10/search/wsdl/GetEventSearchResults"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="FindPTZPosition">
			<soap:operation soapAction="http://www.onvif.org/ver10/search/wsdl/FindPTZPosition"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetPTZPositionSearchResults">
			<soap:operation soapAction="http://www.onvif.org/ver10/search/wsdl/GetPTZPositionSearchResults"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetSearchState">
			<soap:operation soapAction="http://www.onvif.org/ver10/search/wsdl/GetSearchState"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="EndSearch">
			<soap:operation soapAction="http://www.onvif.org/ver10/search/wsdl/EndSearch"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="FindMetadata">
			<soap:operation soapAction="http://www.onvif.org/ver10/search/wsdl/FindMetadata"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<wsdl:operation name="GetMetadataSearchResults">
			<soap:operation soapAction="http://www.onvif.org/ver10/search/wsdl/GetMetadataSearchResults"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<!--===============================-->
		<!--===============================-->
	</wsdl:binding>
</wsdl:definitions>
